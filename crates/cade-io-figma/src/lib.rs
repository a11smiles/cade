//! Figma import through the Figma REST API with a user-supplied token.
//!
//! The `.fig` format is undocumented, so CADE reads files over the API:
//! `GET /v1/files/:key` for the node tree, `/v1/files/:key/variables/local`
//! for variables (mapped to tokens) and `/v1/images/:key` for raster fills.
//! The token is kept in the OS keychain (ONB) and sent only to api.figma.com.

use url::Url;

pub const API: &str = "https://api.figma.com";

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum FigmaError {
    #[error("not a Figma file link")]
    NotAFileLink,
}

/// Pull the file key out of a figma.com/file/... or figma.com/design/... link.
pub fn file_key(link: &str) -> Result<String, FigmaError> {
    let u = Url::parse(link).map_err(|_| FigmaError::NotAFileLink)?;
    if !u.host_str().is_some_and(|h| h == "figma.com" || h.ends_with(".figma.com")) {
        return Err(FigmaError::NotAFileLink);
    }
    let mut seg = u.path_segments().ok_or(FigmaError::NotAFileLink)?;
    match (seg.next(), seg.next()) {
        (Some("file" | "design"), Some(key)) if !key.is_empty() => Ok(key.to_string()),
        _ => Err(FigmaError::NotAFileLink),
    }
}

pub fn file_url(key: &str) -> String {
    format!("{API}/v1/files/{key}")
}

pub fn variables_url(key: &str) -> String {
    format!("{API}/v1/files/{key}/variables/local")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extracts_keys() {
        assert_eq!(
            file_key("https://www.figma.com/design/AbC123/Claims-Portal?node-id=1-2").unwrap(),
            "AbC123"
        );
        assert_eq!(file_key("https://figma.com/file/XyZ/Old").unwrap(), "XyZ");
        assert!(file_key("https://example.com/design/AbC").is_err());
    }
}
