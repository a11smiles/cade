//! Print one generated artifact or a private test-runner preview envelope.
//! Bindings may also be written to an explicit output path by `just bindings`.
//! Ordinary tests only compare artifacts and never write tracked files.

mod support;

use cade_feedback::Feedback;
use serde_json::{Value, json};

fn preview(input: Value) -> Value {
    let Ok(feedback) = serde_json::from_value::<Feedback>(input) else {
        // Deserializer diagnostics may include input; never print them.
        return json!({"error": {"Message": "invalid feedback JSON"}});
    };
    match feedback.to_issue() {
        Ok(issue) => json!({"ok": issue}),
        Err(error) => json!({"error": {"Message": error.to_string()}}),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match std::env::args().nth(1).as_deref() {
        Some("bindings") => match std::env::args_os().nth(2) {
            Some(path) => std::fs::write(path, support::bindings())?,
            None => print!("{}", support::bindings()),
        },
        Some("forms") => print!("{}", support::forms()?),
        Some("preview") => {
            let envelope = match serde_json::from_reader::<_, Value>(std::io::stdin().lock()) {
                Ok(input) => preview(input),
                Err(_) => json!({"error": {"Message": "invalid feedback JSON"}}),
            };
            serde_json::to_writer(std::io::stdout().lock(), &envelope).map_err(|_| "could not write feedback preview")?;
        }
        _ => return Err("usage: cargo run -p cade-feedback --example generate_feedback -- bindings [output-path]|forms|preview".into()),
    }
    Ok(())
}
