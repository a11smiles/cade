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
use std::collections::BTreeMap;
use std::sync::LazyLock;
use ts_rs::TS;

mod answers;
mod forms;

/// The hosted issue form used to collect and label this feedback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "kebab-case")]
pub enum FeedbackKind {
    Bug,
    FeatureRequest,
    Comment,
}

impl FeedbackKind {
    /// Labels applied on creation. They match `.github/labels.yml`.
    pub fn labels(self) -> &'static [&'static str] {
        forms::labels(self)
    }

    /// The issue form this kind mirrors.
    pub fn template(self) -> &'static str {
        match self {
            FeedbackKind::Bug => "bug.yml",
            FeedbackKind::FeatureRequest => "feature_request.yml",
            FeedbackKind::Comment => "comment.yml",
        }
    }

    /// Headings of the answers this kind's issue form marks `required: true`, in form order.
    pub fn required_headings(self) -> &'static [&'static str] {
        forms::required_headings(self)
    }

    /// Title prefix used by the issue forms.
    pub fn title_prefix(self) -> &'static str {
        forms::text(&forms::form(self)["title"])
    }
}

/// One legacy heading/text answer, retained for migration of older feedback.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct Answer {
    pub heading: String,
    pub text: String,
}

/// Environment details. Each is included only if the user leaves it checked.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, TS)]
pub struct Environment {
    #[serde(alias = "version")]
    #[ts(optional = nullable)]
    pub app_version: Option<String>,
    #[ts(optional = nullable)]
    pub os: Option<String>,
    #[ts(optional = nullable)]
    pub gpu: Option<String>,
    #[ts(optional = nullable)]
    pub policy_version: Option<String>,
    /// Already-scrubbed log tail. Off by default.
    #[ts(optional = nullable)]
    pub logs: Option<String>,
}

/// Approved form answers and opt-in details, validated and redacted before sending.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct Feedback {
    pub kind: FeedbackKind,
    pub title: String,
    /// Canonical hosted-form IDs. Legacy heading/text arrays migrate on input.
    #[serde(deserialize_with = "answers::deserialize")]
    pub answers: BTreeMap<String, String>,
    pub environment: Environment,
    /// Asset URL of an uploaded screenshot the user approved.
    #[ts(optional = nullable)]
    pub screenshot_url: Option<String>,
}

/// What `cade-github` sends to `POST /repos/{owner}/{repo}/issues`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct IssueDraft {
    pub title: String,
    pub body: String,
    pub labels: Vec<String>,
}

/// An actionable validation failure; no issue is publishable until it is resolved.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum FeedbackError {
    #[error("a title is required")]
    MissingTitle,
    #[error("\"{0}\" is required")]
    MissingAnswer(String),
    /// The caller supplied a field outside the selected form.
    #[error("invalid answer: {0}")]
    InvalidAnswer(String),
    #[error("title is longer than 256 characters")]
    TitleTooLong,
    #[error("a screenshot must be an https URL")]
    ScreenshotNotHttps,
}

impl Feedback {
    /// Validate, redact and lay out the issue.
    pub fn to_issue(&self) -> Result<IssueDraft, FeedbackError> {
        let title = self.title.trim();
        if title.is_empty() {
            return Err(FeedbackError::MissingTitle);
        }
        for field in forms::fields(self.kind) {
            if field["validations"]["required"].as_bool() == Some(true)
                && !self
                    .answers
                    .get(forms::text(&field["id"]))
                    .is_some_and(|text| !text.trim().is_empty())
            {
                return Err(FeedbackError::MissingAnswer(forms::text(&field["attributes"]["label"]).into()));
            }
        }
        if self
            .answers
            .keys()
            .any(|id| !forms::fields(self.kind).any(|field| field["id"].as_str() == Some(id.as_str())))
        {
            return Err(FeedbackError::InvalidAnswer(
                "use only the field IDs in the selected feedback form".into(),
            ));
        }
        let screenshot = match &self.screenshot_url {
            Some(url) if !url.get(..8).is_some_and(|scheme| scheme.eq_ignore_ascii_case("https://")) => {
                return Err(FeedbackError::ScreenshotNotHttps);
            }
            Some(url) => Some(redact(url)),
            None => None,
        };
        let full_title = format!("{}{}", self.kind.title_prefix(), redact(title));
        if full_title.chars().count() > 256 {
            return Err(FeedbackError::TitleTooLong);
        }

        let mut body = String::new();
        for field in forms::fields(self.kind) {
            body.push_str(&format!("### {}\n\n", forms::text(&field["attributes"]["label"])));
            let t = answers::canonical(field, self.answers.get(forms::text(&field["id"])).map_or("", String::as_str))?;
            body.push_str(if t.is_empty() { "_No response_" } else { "" });
            body.push_str(&redact(&t));
            body.push_str("\n\n");
        }
        if let Some(url) = screenshot {
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
/// Prefixes distinctive enough to match even when glued to a preceding word
/// character (`X_ghp_…`, `TOKENgithub_pat_…`).
static PREFIXED_SECRETS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(gh[pousr]_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|xox[abp]-[A-Za-z0-9-]{10,})\b").unwrap());
/// Prefixes that also occur inside ordinary words (`risk-`), so they need a word boundary.
static BOUNDED_SECRETS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(sk-[A-Za-z0-9_-]{20,}|AKIA[0-9A-Z]{16})\b").unwrap());
static BEARER_HEADER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)(authorization:\s*bearer\s+)\S+").unwrap());
static BEARER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\b(bearer\s+)([A-Za-z0-9\-._~+/]+=*)").unwrap());
/// Azure OpenAI and similar `api-key` values: 32 hex digits after a key name.
static API_KEY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?i)\b(api[-_]?key["']?\s*[:=]\s*["']?)[0-9a-f]{32}\b"#).unwrap());
static EMAIL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b").unwrap());

/// A bearer value is a token, not a word, when it's long and has a non-letter in it.
fn is_token(value: &str) -> bool {
    value.len() >= 8 && value.chars().any(|c| !c.is_ascii_alphabetic())
}

/// Remove things that must never leave the machine in feedback: user home
/// paths, tokens and keys, bearer tokens, API key values and email addresses.
pub fn redact(text: &str) -> String {
    let t = HOME_PATHS.replace_all(text, "${1}<user>");
    let t = PREFIXED_SECRETS.replace_all(&t, "<redacted-secret>");
    let t = BOUNDED_SECRETS.replace_all(&t, "<redacted-secret>");
    let t = BEARER_HEADER.replace_all(&t, "${1}<redacted>");
    let t = BEARER.replace_all(&t, |c: &regex::Captures<'_>| {
        if is_token(&c[2]) {
            format!("{}<redacted>", &c[1])
        } else {
            c[0].to_string()
        }
    });
    let t = API_KEY.replace_all(&t, "${1}<redacted>");
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
            answers: BTreeMap::from([
                (
                    "what-happened".into(),
                    "Run stopped at step 3 in /Users/jdavis/design/claims".into(),
                ),
                ("steps".into(), "1. Open\n2. Ask".into()),
                ("expected".into(), "".into()),
            ]),
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
        assert_eq!(i.labels, vec!["bug", "needs-triage", "type:bug"]);
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
        f.answers.get_mut("what-happened").unwrap().clear();
        assert!(matches!(f.to_issue(), Err(FeedbackError::MissingAnswer(_))));
    }

    #[test]
    fn each_kind_maps_to_a_template_and_labels() {
        assert_eq!(FeedbackKind::FeatureRequest.template(), "feature_request.yml");
        assert_eq!(FeedbackKind::Comment.labels(), &["feedback", "type:comment"]);
    }

    fn comment(title: &str, text: &str) -> Feedback {
        Feedback {
            kind: FeedbackKind::Comment,
            title: title.into(),
            answers: BTreeMap::from([("comment".into(), text.into())]),
            environment: Environment::default(),
            screenshot_url: None,
        }
    }

    #[test]
    fn labels_bug_and_feature_request_match_issue_forms() {
        assert_eq!(FeedbackKind::Bug.labels(), &["bug", "needs-triage", "type:bug"]);
        assert_eq!(
            FeedbackKind::FeatureRequest.labels(),
            &["enhancement", "needs-triage", "type:feature"]
        );
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
                body: "### Your comment\n\nFast on 10k layers.\n\n\
                       ### Where in CADE?\n\n_No response_\n\n\
                       ### Output type\n\n_No response_\n\n\
                       ### Would you like a reply?\n\n_No response_\n\n\
                       <sub>Sent from CADE (Help › Send feedback).</sub>\n"
                    .into(),
                labels: vec!["feedback".into(), "type:comment".into()],
            }
        );
    }

    #[test]
    fn to_issue_feature_request_uses_feature_prefix_and_enhancement_label() {
        let i = feature("Needed for motion", "Export Lottie files").to_issue().unwrap();
        assert_eq!(i.title, "[Feature] Export to Lottie");
        assert_eq!(i.labels, vec!["enhancement", "needs-triage", "type:feature"]);
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

    fn feature(problem: &str, wish: &str) -> Feedback {
        Feedback {
            kind: FeedbackKind::FeatureRequest,
            title: "Export to Lottie".into(),
            answers: BTreeMap::from([("problem".into(), problem.into()), ("proposal".into(), wish.into())]),
            environment: Environment::default(),
            screenshot_url: None,
        }
    }

    #[test]
    fn to_issue_empty_second_required_bug_answer_returns_missing_answer() {
        let mut f = bug();
        f.answers.insert("steps".into(), " \n ".into());
        assert_eq!(f.to_issue(), Err(FeedbackError::MissingAnswer("Steps to reproduce".into())));
    }

    #[test]
    fn to_issue_required_answer_left_out_returns_missing_answer() {
        let mut f = feature("Handoff to motion", "x");
        f.answers.remove("proposal");
        assert_eq!(
            f.to_issue(),
            Err(FeedbackError::MissingAnswer("What would you like CADE to do?".into()))
        );
    }

    #[test]
    fn required_headings_each_kind_lists_its_forms_required_answers() {
        assert_eq!(FeedbackKind::Bug.required_headings(), &["What happened?", "Steps to reproduce"]);
        assert_eq!(
            FeedbackKind::FeatureRequest.required_headings(),
            &["What problem would this solve?", "What would you like CADE to do?"]
        );
        assert_eq!(FeedbackKind::Comment.required_headings(), &["Your comment"]);
    }

    #[test]
    fn to_issue_file_screenshot_url_returns_screenshot_not_https() {
        let mut f = comment("t", "x");
        f.screenshot_url = Some("file:///Users/ann/shot.png".into());
        assert_eq!(f.to_issue(), Err(FeedbackError::ScreenshotNotHttps));
    }

    #[test]
    fn to_issue_http_screenshot_url_returns_screenshot_not_https() {
        let mut f = comment("t", "x");
        f.screenshot_url = Some("http://example.com/shot.png".into());
        assert_eq!(f.to_issue(), Err(FeedbackError::ScreenshotNotHttps));
    }

    #[test]
    fn to_issue_https_screenshot_url_is_redacted() {
        let mut f = comment("t", "x");
        f.screenshot_url = Some("https://example.com/shot.png?token=ghp_A1b2C3d4E5f6G7h8I9j0K1".into());
        let body = f.to_issue().unwrap().body;
        assert!(
            body.contains("![screenshot](https://example.com/shot.png?token=<redacted-secret>)"),
            "{body}"
        );
        assert!(!body.contains("ghp_"), "{body}");
    }

    #[test]
    fn redact_bare_bearer_token_is_redacted() {
        assert_eq!(redact("sent Bearer abc.def.ghi to the API"), "sent Bearer <redacted> to the API");
        assert_eq!(redact("bearer eyJhbGciOiJIUzI1NiJ9"), "bearer <redacted>");
    }

    #[test]
    fn redact_bearer_in_prose_is_left_alone() {
        let s = "The bearer of the badge and Bearer instruments stay.";
        assert_eq!(redact(s), s);
    }

    #[test]
    fn redact_token_after_word_character_is_redacted() {
        assert_eq!(redact("key=X_ghp_A1b2C3d4E5f6G7h8I9j0K1L2m3N4o5P6q7"), "key=X_<redacted-secret>");
        assert_eq!(redact("TOKENgithub_pat_11ABCDEFG0123456789_abc"), "TOKEN<redacted-secret>");
    }

    #[test]
    fn redact_azure_api_key_value_is_redacted() {
        assert_eq!(redact("api-key: 0123456789abcdef0123456789ABCDEF"), "api-key: <redacted>");
        assert_eq!(
            redact(r#"{"api_key": "0123456789abcdef0123456789abcdef"}"#),
            r#"{"api_key": "<redacted>"}"#
        );
    }

    #[test]
    fn redact_hex_without_key_name_is_left_alone() {
        let s = "commit 7b0f9e0c1d2e3f405162738495a6b7c8d9e0f1a2 md5 0123456789abcdef0123456789abcdef";
        assert_eq!(redact(s), s);
    }

    #[test]
    fn redact_word_ending_in_sk_before_dash_is_left_alone() {
        let s = "risk-management-assessment-v2 and disk-0123456789abcdefghij";
        assert_eq!(redact(s), s);
    }
}
