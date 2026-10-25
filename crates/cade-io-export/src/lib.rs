//! Exporters for every output type.
//!
//! Each exporter turns a rendered document into bytes in one format. The
//! formats and which output types offer them live here so the UI's Export
//! menus and the backlog stay in one place.

use cade_core::OutputKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Png,
    Svg,
    Pdf,
    Pptx,
    Docx,
    Markdown,
    Mp4,
    Gif,
    Lottie,
    Css,
    Vsdx,
    Mermaid,
    Bicep,
    Terraform,
    Sql,
}

pub fn formats_for(kind: OutputKind) -> &'static [Format] {
    use Format::*;
    match kind {
        OutputKind::App => &[Png, Svg, Pdf],
        OutputKind::Image => &[Png, Svg, Pdf],
        OutputKind::Deck => &[Pptx, Pdf, Png],
        OutputKind::Document => &[Pdf, Docx, Markdown],
        OutputKind::Motion => &[Mp4, Gif, Lottie, Css],
        OutputKind::Diagram => &[Vsdx, Svg, Pdf, Png, Mermaid],
        OutputKind::Architecture => &[Bicep, Terraform, Svg, Png],
        OutputKind::DataModel => &[Sql, Svg, Png],
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("{0:?} is not available for this output type")]
    Unsupported(Format),
}

pub trait Exporter {
    fn format(&self) -> Format;
    fn export(&self, scene: &[u8]) -> Result<Vec<u8>, ExportError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_kind_exports_something() {
        for k in cade_core::OutputKind::ALL {
            assert!(!super::formats_for(k).is_empty());
        }
    }

    use super::*;
    use cade_core::OutputKind;

    #[test]
    fn formats_for_each_kind_returns_menu_in_order() {
        use Format::*;
        let expected: [(OutputKind, &[Format]); 8] = [
            (OutputKind::App, &[Png, Svg, Pdf]),
            (OutputKind::Image, &[Png, Svg, Pdf]),
            (OutputKind::Deck, &[Pptx, Pdf, Png]),
            (OutputKind::Document, &[Pdf, Docx, Markdown]),
            (OutputKind::Motion, &[Mp4, Gif, Lottie, Css]),
            (OutputKind::Diagram, &[Vsdx, Svg, Pdf, Png, Mermaid]),
            (OutputKind::Architecture, &[Bicep, Terraform, Svg, Png]),
            (OutputKind::DataModel, &[Sql, Svg, Png]),
        ];
        for (kind, formats) in expected {
            assert_eq!(formats_for(kind), formats, "{kind:?}");
        }
    }

    #[test]
    fn formats_for_infra_and_sql_formats_belong_to_one_kind_each() {
        for (format, owner) in [
            (Format::Bicep, OutputKind::Architecture),
            (Format::Terraform, OutputKind::Architecture),
            (Format::Sql, OutputKind::DataModel),
            (Format::Vsdx, OutputKind::Diagram),
        ] {
            let kinds: Vec<OutputKind> = OutputKind::ALL.into_iter().filter(|k| formats_for(*k).contains(&format)).collect();
            assert_eq!(kinds, vec![owner], "{format:?}");
        }
    }

    #[test]
    fn export_error_unsupported_message_names_format() {
        assert_eq!(
            ExportError::Unsupported(Format::Lottie).to_string(),
            "Lottie is not available for this output type"
        );
    }
}
