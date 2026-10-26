//! Embedded issue forms are the authority for labels, headings and field order.

use crate::FeedbackKind;
use serde_yaml_ng::Value;
use std::sync::LazyLock;

const KINDS: [FeedbackKind; 3] = [FeedbackKind::Bug, FeedbackKind::FeatureRequest, FeedbackKind::Comment];

static FORMS: LazyLock<[Value; 3]> = LazyLock::new(|| {
    [
        include_str!("../../../.github/ISSUE_TEMPLATE/bug.yml"),
        include_str!("../../../.github/ISSUE_TEMPLATE/feature_request.yml"),
        include_str!("../../../.github/ISSUE_TEMPLATE/comment.yml"),
    ]
    .map(|yaml| serde_yaml_ng::from_str(yaml).expect("embedded issue forms are valid YAML, checked by issue_forms tests"))
});

static LABELS: LazyLock<[Vec<&'static str>; 3]> = LazyLock::new(|| {
    KINDS.map(|kind| {
        form(kind)["labels"]
            .as_sequence()
            .expect("embedded issue forms define labels")
            .iter()
            .map(|label| label.as_str().expect("embedded issue form labels are strings"))
            .collect()
    })
});

static REQUIRED_HEADINGS: LazyLock<[Vec<&'static str>; 3]> = LazyLock::new(|| {
    KINDS.map(|kind| {
        fields(kind)
            .filter(|field| field["validations"]["required"].as_bool() == Some(true))
            .map(|field| text(&field["attributes"]["label"]))
            .collect()
    })
});

fn index(kind: FeedbackKind) -> usize {
    match kind {
        FeedbackKind::Bug => 0,
        FeedbackKind::FeatureRequest => 1,
        FeedbackKind::Comment => 2,
    }
}

pub(crate) fn form(kind: FeedbackKind) -> &'static Value {
    &FORMS[index(kind)]
}

pub(crate) fn labels(kind: FeedbackKind) -> &'static [&'static str] {
    &LABELS[index(kind)]
}

pub(crate) fn required_headings(kind: FeedbackKind) -> &'static [&'static str] {
    &REQUIRED_HEADINGS[index(kind)]
}

pub(crate) fn text(value: &Value) -> &str {
    value.as_str().expect("embedded issue form IDs, headings and titles are strings")
}

pub(crate) fn fields(kind: FeedbackKind) -> impl Iterator<Item = &'static Value> {
    form(kind)["body"]
        .as_sequence()
        .expect("embedded issue forms define a body")
        .iter()
        .filter(|field| {
            field["id"]
                .as_str()
                .is_some_and(|id| !matches!(id, "app_version" | "os" | "gpu" | "policy_version" | "screenshot_url"))
        })
}

pub(crate) fn legacy_id(heading: &str) -> Option<&'static str> {
    KINDS
        .into_iter()
        .flat_map(fields)
        .find(|field| field["attributes"]["label"].as_str() == Some(heading))
        .map(|field| text(&field["id"]))
}
