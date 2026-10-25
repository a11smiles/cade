//! In-app feedback: bug reports, feature requests and comments, submitted as
//! GitHub issues on the CADE repository with the signed-in user's identity.
//!
//! This crate builds the issue (title, body, labels) and redacts it. Sending
//! is `cade-github`'s job. The body layout matches the issue forms in
//! `.github/ISSUE_TEMPLATE/`, so issues filed in the app and on github.com
//! read the same.
//!
//! Backlog: FEED epic.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FeedbackKind {
    Bug,
    FeatureRequest,
    Comment,
}

impl FeedbackKind {
    /// Labels applied on creation. They match `.github/labels.yml`.
    pub fn labels(self) -> &'static [&'static str] {
        match self {
            FeedbackKind::Bug => &["bug", "needs-triage"],
            FeedbackKind::FeatureRequest => &["enhancement"],
            FeedbackKind::Comment => &["feedback"],
        }
    }

    /// The issue form this kind mirrors.
    pub fn template(self) -> &'static str {
        match self {
            FeedbackKind::Bug => "bug.yml",
            FeedbackKind::FeatureRequest => "feature_request.yml",
            FeedbackKind::Comment => "comment.yml",
        }
    }

    /// Title prefix used by the issue forms.
    pub fn title_prefix(self) -> &'static str {
        match self {
            FeedbackKind::Bug => "[Bug] ",
            FeedbackKind::FeatureRequest => "[Feature] ",
            FeedbackKind::Comment => "[Comment] ",
        }
    }
}

/// One answered question from the form, in display order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Answer {
    pub heading: String,
    pub text: String,
}

/// Environment details. Each is included only if the user leaves it checked.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Environment {
    pub app_version: Option<String>,
    pub os: Option<String>,
    pub gpu: Option<String>,
    pub policy_version: Option<String>,
    /// Already-scrubbed log tail. Off by default.
    pub logs: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feedback {
    pub kind: FeedbackKind,
    pub title: String,
    pub answers: Vec<Answer>,
    pub environment: Environment,
    /// Asset URL of an uploaded screenshot the user approved.
    pub screenshot_url: Option<String>,
}

/// What `cade-github` sends to `POST /repos/{owner}/{repo}/issues`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueDraft {
    pub title: String,
    pub body: String,
    pub labels: Vec<String>,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum FeedbackError {
    #[error("a title is required")]
    MissingTitle,
    #[error("\"{0}\" is required")]
    MissingAnswer(String),
    #[error("title is longer than 256 characters")]
    TitleTooLong,
}

impl Feedback {
    /// Validate, redact and lay out the issue.
    pub fn to_issue(&self) -> Result<IssueDraft, FeedbackError> {
        let title = self.title.trim();
        if title.is_empty() {
            return Err(FeedbackError::MissingTitle);
        }
        if let Some(a) = self.answers.first()
            && a.text.trim().is_empty()
        {
            return Err(FeedbackError::MissingAnswer(a.heading.clone()));
        }
        let full_title = format!("{}{}", self.kind.title_prefix(), redact(title));
        if full_title.chars().count() > 256 {
            return Err(FeedbackError::TitleTooLong);
        }

        let mut body = String::new();
        for a in &self.answers {
            body.push_str(&format!("### {}\n\n", a.heading));
            let t = a.text.trim();
            body.push_str(if t.is_empty() { "_No response_" } else { "" });
            body.push_str(&redact(t));
            body.push_str("\n\n");
        }
        if let Some(url) = &self.screenshot_url {
            body.push_str(&format!("### Screenshot\n\n![screenshot]({url})\n\n"));
        }
        let env = &self.environment;
        let rows: Vec<(&str, &Option<String>)> = vec![
            ("CADE version", &env.app_version),
            ("Operating system", &env.os),
            ("GPU and renderer", &env.gpu),
            ("Policy version", &env.policy_version),
        ];
        let present: Vec<_> = rows.into_iter().filter_map(|(k, v)| v.as_ref().map(|v| (k, v))).collect();
        if !present.is_empty() || env.logs.is_some() {
            body.push_str("<details><summary>Environment</summary>\n\n");
            for (k, v) in present {
                body.push_str(&format!("- **{k}:** {}\n", redact(v)));
            }
            if let Some(logs) = &env.logs {
                body.push_str(&format!("\n```text\n{}\n```\n", redact(logs)));
            }
            body.push_str("\n</details>\n\n");
        }
        body.push_str("<sub>Sent from CADE (Help › Send feedback).</sub>\n");

        Ok(IssueDraft {
            title: full_title,
            body,
            labels: self.kind.labels().iter().map(|s| s.to_string()).collect(),
        })
    }
}

static HOME_PATHS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)(/Users/|/home/|[A-Z]:\\Users\\)[^/\\\s]+").unwrap());
static SECRETS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\b(gh[pousr]_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|sk-[A-Za-z0-9_-]{20,}|xox[abp]-[A-Za-z0-9-]{10,}|AKIA[0-9A-Z]{16})\b",
    )
    .unwrap()
});
static BEARER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)(authorization:\s*bearer\s+)\S+").unwrap());
static EMAIL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b").unwrap());

/// Remove things that must never leave the machine in feedback: user home
/// paths, tokens and keys, bearer headers and email addresses.
pub fn redact(text: &str) -> String {
    let t = HOME_PATHS.replace_all(text, "${1}<user>");
    let t = SECRETS.replace_all(&t, "<redacted-secret>");
    let t = BEARER.replace_all(&t, "${1}<redacted>");
    let t = EMAIL.replace_all(&t, "<email>");
    t.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bug() -> Feedback {
        Feedback {
            kind: FeedbackKind::Bug,
            title: "Contrast check times out".into(),
            answers: vec![
                Answer {
                    heading: "What happened?".into(),
                    text: "Run stopped at step 3 in /Users/jdavis/design/claims".into(),
                },
                Answer {
                    heading: "Steps to reproduce".into(),
                    text: "1. Open\n2. Ask".into(),
                },
                Answer {
                    heading: "What did you expect?".into(),
                    text: "".into(),
                },
            ],
            environment: Environment {
                app_version: Some("0.9.0".into()),
                os: Some("macOS 26.1".into()),
                ..Default::default()
            },
            screenshot_url: None,
        }
    }

    #[test]
    fn builds_a_labeled_issue() {
        let i = bug().to_issue().unwrap();
        assert_eq!(i.title, "[Bug] Contrast check times out");
        assert_eq!(i.labels, vec!["bug", "needs-triage"]);
        assert!(i.body.contains("### Steps to reproduce"));
        assert!(i.body.contains("_No response_"));
        assert!(i.body.contains("**CADE version:** 0.9.0"));
        assert!(!i.body.contains("GPU"));
    }

    #[test]
    fn redacts_paths_secrets_and_emails() {
        let i = bug().to_issue().unwrap();
        assert!(i.body.contains("/Users/<user>/design"));
        let r = redact("token ghp_abcdefghijklmnopqrstuvwxyz123456 by a@b.co in C:\\Users\\Ann\\x");
        assert!(r.contains("<redacted-secret>"));
        assert!(r.contains("<email>"));
        assert!(r.contains("C:\\Users\\<user>\\x"));
        assert_eq!(redact("Authorization: Bearer abc.def"), "Authorization: Bearer <redacted>");
    }

    #[test]
    fn requires_title_and_first_answer() {
        let mut f = bug();
        f.title = "  ".into();
        assert_eq!(f.to_issue(), Err(FeedbackError::MissingTitle));
        let mut f = bug();
        f.answers[0].text.clear();
        assert!(matches!(f.to_issue(), Err(FeedbackError::MissingAnswer(_))));
    }

    #[test]
    fn each_kind_maps_to_a_template_and_labels() {
        assert_eq!(FeedbackKind::FeatureRequest.template(), "feature_request.yml");
        assert_eq!(FeedbackKind::Comment.labels(), &["feedback"]);
    }
}
