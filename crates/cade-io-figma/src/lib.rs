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

    #[test]
    fn file_key_design_link_with_query_and_fragment_returns_key() {
        assert_eq!(
            file_key("https://www.figma.com/design/AbC123/Claims-Portal?node-id=1-2&t=x#frag").unwrap(),
            "AbC123"
        );
    }

    #[test]
    fn file_key_link_without_title_segment_returns_key() {
        assert_eq!(file_key("https://www.figma.com/file/KeyOnly").unwrap(), "KeyOnly");
        assert_eq!(file_key("https://www.figma.com/design/KeyOnly/").unwrap(), "KeyOnly");
    }

    #[test]
    fn file_key_subdomain_of_figma_com_is_accepted() {
        assert_eq!(file_key("https://staging.figma.com/design/Sub1/x").unwrap(), "Sub1");
    }

    #[test]
    fn file_key_lookalike_hosts_return_not_a_file_link() {
        for link in [
            "https://notfigma.com/design/AbC/x",
            "https://figma.com.evil.example/design/AbC/x",
            "https://evil.example/figma.com/design/AbC/x",
        ] {
            assert_eq!(file_key(link), Err(FigmaError::NotAFileLink), "{link}");
        }
    }

    #[test]
    fn file_key_other_figma_paths_return_not_a_file_link() {
        for link in [
            "https://www.figma.com/proto/AbC/x",
            "https://www.figma.com/board/AbC/x",
            "https://www.figma.com/community/file/123",
            "https://www.figma.com/",
            "https://www.figma.com/design/",
            "https://www.figma.com/design//x",
        ] {
            assert_eq!(file_key(link), Err(FigmaError::NotAFileLink), "{link}");
        }
    }

    #[test]
    fn file_key_unparseable_input_returns_not_a_file_link() {
        for link in ["", "figma.com/design/AbC/x", "AbC123", "mailto:someone@figma.com"] {
            assert_eq!(file_key(link), Err(FigmaError::NotAFileLink), "{link:?}");
        }
    }

    #[test]
    fn file_url_key_returns_files_endpoint() {
        assert_eq!(file_url("AbC123"), "https://api.figma.com/v1/files/AbC123");
    }

    #[test]
    fn variables_url_key_returns_local_variables_endpoint() {
        assert_eq!(variables_url("AbC123"), "https://api.figma.com/v1/files/AbC123/variables/local");
    }

    #[test]
    fn figma_error_not_a_file_link_message_is_plain() {
        assert_eq!(FigmaError::NotAFileLink.to_string(), "not a Figma file link");
    }
}
