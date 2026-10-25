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

    fn comment(title: &str, text: &str) -> Feedback {
        Feedback {
            kind: FeedbackKind::Comment,
            title: title.into(),
            answers: vec![Answer {
                heading: "Your comment".into(),
                text: text.into(),
            }],
            environment: Environment::default(),
            screenshot_url: None,
        }
    }

    #[test]
    fn labels_bug_and_feature_request_match_issue_forms() {
        assert_eq!(FeedbackKind::Bug.labels(), &["bug", "needs-triage"]);
        assert_eq!(FeedbackKind::FeatureRequest.labels(), &["enhancement"]);
    }

    #[test]
    fn template_bug_and_comment_name_their_forms() {
        assert_eq!(FeedbackKind::Bug.template(), "bug.yml");
        assert_eq!(FeedbackKind::Comment.template(), "comment.yml");
    }

    #[test]
    fn title_prefix_each_kind_matches_its_form_title() {
        assert_eq!(FeedbackKind::Bug.title_prefix(), "[Bug] ");
        assert_eq!(FeedbackKind::FeatureRequest.title_prefix(), "[Feature] ");
        assert_eq!(FeedbackKind::Comment.title_prefix(), "[Comment] ");
    }

    #[test]
    fn to_issue_comment_without_environment_lays_out_exact_body() {
        let i = comment("  Love the canvas  ", "  Fast on 10k layers.  ").to_issue().unwrap();
        assert_eq!(
            i,
            IssueDraft {
                title: "[Comment] Love the canvas".into(),
                body: "### Your comment\n\nFast on 10k layers.\n\n<sub>Sent from CADE (Help › Send feedback).</sub>\n".into(),
                labels: vec!["feedback".into()],
            }
        );
    }

    #[test]
    fn to_issue_feature_request_uses_feature_prefix_and_enhancement_label() {
        let mut f = comment("Export to Lottie", "Needed for motion");
        f.kind = FeedbackKind::FeatureRequest;
        let i = f.to_issue().unwrap();
        assert_eq!(i.title, "[Feature] Export to Lottie");
        assert_eq!(i.labels, vec!["enhancement"]);
    }

    #[test]
    fn to_issue_whitespace_only_first_answer_returns_missing_answer_with_heading() {
        assert_eq!(
            comment("Title", " \n\t ").to_issue(),
            Err(FeedbackError::MissingAnswer("Your comment".into()))
        );
    }

    #[test]
    fn to_issue_title_at_256_characters_with_prefix_is_accepted() {
        let prefix = FeedbackKind::Comment.title_prefix().chars().count();
        let title = "é".repeat(256 - prefix);
        assert_eq!(comment(&title, "x").to_issue().unwrap().title.chars().count(), 256);
    }

    #[test]
    fn to_issue_title_over_256_characters_with_prefix_returns_title_too_long() {
        let prefix = FeedbackKind::Comment.title_prefix().chars().count();
        let title = "é".repeat(257 - prefix);
        assert_eq!(comment(&title, "x").to_issue(), Err(FeedbackError::TitleTooLong));
    }

    #[test]
    fn to_issue_title_is_redacted() {
        let i = comment("Crash for ann@contoso.com", "x").to_issue().unwrap();
        assert_eq!(i.title, "[Comment] Crash for <email>");
    }

    #[test]
    fn to_issue_screenshot_url_adds_screenshot_section() {
        let mut f = comment("t", "x");
        f.screenshot_url = Some("https://github.com/user-attachments/assets/abc".into());
        let body = f.to_issue().unwrap().body;
        assert!(
            body.contains("\n\n### Screenshot\n\n![screenshot](https://github.com/user-attachments/assets/abc)\n\n<sub>"),
            "{body}"
        );
    }

    #[test]
    fn to_issue_every_environment_item_opted_out_omits_environment_section() {
        let mut f = bug();
        f.environment = Environment::default();
        assert!(!f.to_issue().unwrap().body.contains("Environment"));
    }

    #[test]
    fn to_issue_all_environment_items_listed_in_order_and_redacted() {
        let mut f = comment("t", "x");
        f.environment = Environment {
            app_version: Some("0.1.0".into()),
            os: Some("Linux at /home/ann/x".into()),
            gpu: Some("Apple M4".into()),
            policy_version: Some("12".into()),
            logs: None,
        };
        let body = f.to_issue().unwrap().body;
        assert!(
            body.contains(
                "<details><summary>Environment</summary>\n\n\
                 - **CADE version:** 0.1.0\n\
                 - **Operating system:** Linux at /home/<user>/x\n\
                 - **GPU and renderer:** Apple M4\n\
                 - **Policy version:** 12\n\
                 \n</details>\n\n"
            ),
            "{body}"
        );
    }

    #[test]
    fn to_issue_logs_only_adds_redacted_code_block_in_environment() {
        let mut f = comment("t", "x");
        f.environment.logs = Some("auth ok ghp_abcdefghijklmnopqrstuvwxyz0123".into());
        let body = f.to_issue().unwrap().body;
        assert!(
            body.contains("<details><summary>Environment</summary>\n\n\n```text\nauth ok <redacted-secret>\n```\n\n</details>"),
            "{body}"
        );
        assert!(!body.contains("ghp_"));
    }

    #[test]
    fn redact_each_github_token_prefix_is_redacted() {
        for p in ["ghp_", "gho_", "ghu_", "ghs_", "ghr_"] {
            let t = format!("{p}{}", "A1b2C3d4E5f6G7h8I9j0K1");
            assert_eq!(redact(&format!("token {t} end")), "token <redacted-secret> end", "{p}");
        }
    }

    #[test]
    fn redact_fine_grained_pat_is_redacted() {
        let pat = "github_pat_11ABCDEFG0123456789_abcdefghijklmnopqrstuvwxyz";
        assert_eq!(redact(pat), "<redacted-secret>");
    }

    #[test]
    fn redact_model_provider_and_cloud_keys_are_redacted() {
        assert_eq!(redact("key=sk-ant-api03-AbCdEfGhIjKlMnOpQrStUv"), "key=<redacted-secret>");
        assert_eq!(redact("OPENAI sk-proj-0123456789abcdefghij"), "OPENAI <redacted-secret>");
        assert_eq!(redact("xoxb-1234567890-abcdef"), "<redacted-secret>");
        assert_eq!(redact("AKIAABCDEFGHIJKLMNOP"), "<redacted-secret>");
    }

    #[test]
    fn redact_short_token_lookalike_is_left_alone() {
        assert_eq!(redact("ghp_short and sk-tiny"), "ghp_short and sk-tiny");
    }

    #[test]
    fn redact_linux_and_lowercase_windows_home_paths_keep_rest_of_path() {
        assert_eq!(redact("/home/ann/designs/a.cade"), "/home/<user>/designs/a.cade");
        assert_eq!(redact(r"c:\users\Ann\Desktop"), r"c:\users\<user>\Desktop");
    }

    #[test]
    fn redact_lowercase_bearer_header_is_redacted() {
        assert_eq!(redact("authorization: bearer eyJ.abc.def"), "authorization: bearer <redacted>");
    }

    #[test]
    fn redact_every_occurrence_is_replaced() {
        assert_eq!(
            redact("a@b.co, c@d.org and /Users/x/1 /Users/y/2"),
            "<email>, <email> and /Users/<user>/1 /Users/<user>/2"
        );
    }

    #[test]
    fn redact_plain_text_is_unchanged() {
        let s = "Step 3: open the Layers panel at 1440 × 900.";
        assert_eq!(redact(s), s);
    }
}
