//! The existing native rejection envelope, shared with standalone binding generation.

/// An application command failure. Keep the externally tagged Message wire envelope.
#[derive(Debug, serde::Serialize, thiserror::Error, ts_rs::TS)]
pub enum CommandError {
    #[error("{0}")]
    Message(String),
}

impl From<String> for CommandError {
    fn from(s: String) -> Self {
        CommandError::Message(s)
    }
}
