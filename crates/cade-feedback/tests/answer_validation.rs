//! Additional red-phase cases. Capture assertion failures before adding validation.

use cade_feedback::{Environment, Feedback, FeedbackError, FeedbackKind};
use serde_json::json;

fn feedback(kind: FeedbackKind, id: &str, value: &str) -> Feedback {
    let mut answers = match kind {
        FeedbackKind::Bug => json!({"what-happened": "Summary", "steps": "Open"}),
        FeedbackKind::FeatureRequest => json!({"problem": "Problem", "proposal": "Proposal"}),
        FeedbackKind::Comment => json!({"comment": "Comment"}),
    };
    answers[id] = json!(value);
    serde_json::from_value(json!({
        "kind": kind,
        "title": "Choices",
        "answers": answers,
        "environment": {},
        "screenshot_url": null
    }))
    .expect("canonical keyed answers deserialize before option validation")
}

#[test]
fn to_issue_output_unknown_choices_return_invalid_answer() {
    for kind in [FeedbackKind::Bug, FeedbackKind::FeatureRequest, FeedbackKind::Comment] {
        for value in ["Not specific", "Unknown", "app", "App, Unknown"] {
            assert!(
                matches!(feedback(kind, "output", value).to_issue(), Err(FeedbackError::InvalidAnswer(_))),
                "{kind:?}: {value:?} must not produce a draft"
            );
        }
    }
}

#[test]
fn to_issue_output_duplicate_selections_return_invalid_answer() {
    for value in ["App, App", "App, System, App", "Other, Other"] {
        assert!(
            matches!(
                feedback(FeedbackKind::Comment, "output", value).to_issue(),
                Err(FeedbackError::InvalidAnswer(_))
            ),
            "{value:?} must not produce a draft"
        );
    }
}

#[test]
fn to_issue_output_malformed_separators_return_invalid_answer() {
    for value in ["App,System", "App; System", "App\nSystem", "App, ", ", App", "App, , System"] {
        assert!(
            matches!(
                feedback(FeedbackKind::Comment, "output", value).to_issue(),
                Err(FeedbackError::InvalidAnswer(_))
            ),
            "{value:?} must not produce a draft"
        );
    }
}

#[test]
fn to_issue_output_reversed_selections_render_in_form_option_order() {
    let issue = feedback(
        FeedbackKind::Comment,
        "output",
        "Other, System, Data model, Architecture, Diagram, Motion, Document, Deck, Image, App",
    )
    .to_issue()
    .expect("all selections are valid even when their incoming order differs");
    assert_eq!(
        issue.body,
        "### Your comment\n\nComment\n\n\
         ### Where in CADE?\n\n_No response_\n\n\
         ### Output type\n\nApp, Image, Deck, Document, Motion, Diagram, Architecture, Data model, System, Other\n\n\
         ### Would you like a reply?\n\n_No response_\n\n\
         <sub>Sent from CADE (Help › Send feedback).</sub>\n"
    );
}

#[test]
fn to_issue_beneficiaries_unknown_or_multiple_choices_return_invalid_answer() {
    for value in ["everyone", "Individual", "individual, team"] {
        assert!(
            matches!(
                feedback(FeedbackKind::FeatureRequest, "beneficiaries", value).to_issue(),
                Err(FeedbackError::InvalidAnswer(_))
            ),
            "{value:?} must not produce a draft"
        );
    }
}

#[test]
fn to_issue_contribution_and_reply_unknown_choices_return_invalid_answer() {
    for (kind, id) in [(FeedbackKind::FeatureRequest, "contribute"), (FeedbackKind::Comment, "reply")] {
        for value in ["Maybe", "yes", "Yes, No", "Not specified"] {
            assert!(
                matches!(feedback(kind, id, value).to_issue(), Err(FeedbackError::InvalidAnswer(_))),
                "{id}: {value:?} must not produce a draft"
            );
        }
    }
}

#[test]
fn to_issue_optional_choices_valid_or_unanswered_produce_drafts() {
    for (kind, id, values) in [
        (
            FeedbackKind::FeatureRequest,
            "beneficiaries",
            vec!["individual", "team", "admin/security", "", " \n "],
        ),
        (FeedbackKind::FeatureRequest, "contribute", vec!["Yes", "No", "", " \n "]),
        (FeedbackKind::Comment, "reply", vec!["Yes", "No", "", " \n "]),
    ] {
        for value in values {
            assert!(feedback(kind, id, value).to_issue().is_ok(), "{id}: {value:?}");
        }
    }
    for value in [
        "App",
        "Image",
        "Deck",
        "Document",
        "Motion",
        "Diagram",
        "Architecture",
        "Data model",
        "System",
        "Other",
        "",
        " \n ",
        "App, System",
    ] {
        for kind in [FeedbackKind::Bug, FeedbackKind::FeatureRequest, FeedbackKind::Comment] {
            assert!(feedback(kind, "output", value).to_issue().is_ok(), "{kind:?}: {value:?}");
        }
    }
}

#[test]
fn from_value_historical_version_migrates_to_existing_app_version() {
    let environment: Environment = serde_json::from_value(json!({"version": "0.1.0"})).expect("historical version input");
    assert_eq!(environment.app_version.as_deref(), Some("0.1.0"));
    let serialized = serde_json::to_value(environment).unwrap();
    assert_eq!(serialized["app_version"], "0.1.0");
    assert!(serialized.get("version").is_none());
}

#[test]
fn from_str_version_and_app_version_together_reject_duplicate_environment_detail() {
    for raw in [
        r#"{"version":"old","app_version":"new"}"#,
        r#"{"app_version":"new","version":"old"}"#,
        r#"{"version":"old","version":"new"}"#,
    ] {
        assert!(
            serde_json::from_str::<Environment>(raw).is_err(),
            "duplicate version details: {raw}"
        );
    }
}
