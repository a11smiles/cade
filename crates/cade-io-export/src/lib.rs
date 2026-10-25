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
}
