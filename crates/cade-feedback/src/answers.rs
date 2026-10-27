//! Migrate legacy answers, preserve duplicate detection and validate hosted choices.

use crate::{Answer, FeedbackError, forms};
use serde::Deserializer;
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Validate choices against the hosted form and render them in its option order.
pub(crate) fn canonical(field: &serde_yaml_ng::Value, value: &str) -> Result<String, FeedbackError> {
    if value.trim().is_empty() {
        return Ok(String::new());
    }
    let Some(options) = field["attributes"]["options"].as_sequence() else {
        return Ok(value.trim().to_owned());
    };
    let selections: Vec<&str> = if field["attributes"]["multiple"].as_bool() == Some(true) {
        value.split(", ").collect()
    } else {
        vec![value]
    };
    let unique: BTreeSet<&str> = selections.iter().copied().collect();
    let unknown = unique
        .iter()
        .any(|selection| !options.iter().any(|option| option.as_str() == Some(*selection)));
    if unique.len() != selections.len() || unknown {
        return Err(FeedbackError::InvalidAnswer(format!(
            "choose each listed option at most once for \"{}\"",
            forms::text(&field["attributes"]["label"])
        )));
    }
    Ok(options
        .iter()
        .map(forms::text)
        .filter(|option| unique.contains(option))
        .collect::<Vec<_>>()
        .join(", "))
}

pub(crate) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<BTreeMap<String, String>, D::Error> {
    struct AnswersVisitor;

    impl<'de> Visitor<'de> for AnswersVisitor {
        type Value = BTreeMap<String, String>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a map of form IDs to strings or legacy heading/text answers")
        }

        fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
            let mut answers = BTreeMap::new();
            while let Some((id, text)) = map.next_entry::<String, String>()? {
                if answers.insert(id, text).is_some() {
                    return Err(M::Error::custom("answer IDs must be unique; keep one answer per field"));
                }
            }
            Ok(answers)
        }

        fn visit_seq<S: SeqAccess<'de>>(self, mut sequence: S) -> Result<Self::Value, S::Error> {
            let mut answers = BTreeMap::new();
            while let Some(answer) = sequence.next_element::<Answer>()? {
                let id = forms::legacy_id(&answer.heading)
                    .ok_or_else(|| S::Error::custom("unknown legacy heading; use the questions in this feedback form"))?;
                if answers.insert(id.to_owned(), answer.text).is_some() {
                    return Err(S::Error::custom("legacy headings must be unique; keep one answer per field"));
                }
            }
            Ok(answers)
        }
    }

    deserializer.deserialize_any(AnswersVisitor)
}
