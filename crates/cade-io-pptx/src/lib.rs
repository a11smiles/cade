//! PowerPoint import and export.
//!
//! Reads the slide list and layout names from a `.pptx`/`.potx` package.
//! Shapes, charts with embedded data, masters and notes are DECK/IMPORT
//! backlog items, as is writing `.pptx` with editable text.

use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::{Read, Seek};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Presentation {
    pub slide_count: usize,
    pub layouts: Vec<String>,
    pub is_template: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum PptxError {
    #[error("not a PowerPoint package")]
    NotPptx,
    #[error("zip: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("xml: {0}")]
    Xml(String),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub fn read_presentation<R: Read + Seek>(reader: R) -> Result<Presentation, PptxError> {
    let mut z = zip::ZipArchive::new(reader)?;
    let names: Vec<String> = (0..z.len())
        .filter_map(|i| z.by_index(i).ok().and_then(|f| f.name().ok().map(|n| n.to_string())))
        .collect();
    if !names.iter().any(|n| n == "ppt/presentation.xml") {
        return Err(PptxError::NotPptx);
    }
    let slide_count = names
        .iter()
        .filter(|n| n.starts_with("ppt/slides/slide") && n.ends_with(".xml"))
        .count();
    let mut layouts = vec![];
    for n in names
        .iter()
        .filter(|n| n.starts_with("ppt/slideLayouts/slideLayout") && n.ends_with(".xml"))
    {
        let mut s = String::new();
        z.by_name(n)?.read_to_string(&mut s)?;
        let mut r = Reader::from_str(&s);
        loop {
            match r.read_event() {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) if e.local_name().as_ref() == "cSld" => {
                    for a in e.attributes().flatten() {
                        if a.key.local_name().as_ref() == "name" {
                            layouts.push(
                                a.normalized_value(quick_xml::XmlVersion::Implicit1_0)
                                    .map_err(|x| PptxError::Xml(x.to_string()))?
                                    .to_string(),
                            );
                        }
                    }
                    break;
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(PptxError::Xml(e.to_string())),
                _ => {}
            }
        }
    }
    let mut ct = String::new();
    if let Ok(mut f) = z.by_name("[Content_Types].xml") {
        f.read_to_string(&mut ct)?;
    }
    Ok(Presentation {
        slide_count,
        layouts,
        is_template: ct.contains("presentationml.template"),
    })
}
