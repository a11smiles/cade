//! Visio import and export.
//!
//! `.vsdx`, `.vssx` (stencil), `.vstx` (template) and macro-enabled `.vsdm`
//! are Open Packaging (zip + XML) files described by the published
//! [MS-VSDX] spec. This module reads the package structure: pages, masters
//! and whether macros are present. Shapes, connectors, Shape Data and the
//! ShapeSheet subset are DIAGRAM backlog items. Legacy binary `.vsd` goes
//! through a converter (DIAGRAM backlog).
//!
//! [MS-VSDX]: https://learn.microsoft.com/openspecs/sharepoint_protocols/ms-vsdx/

use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::{Read, Seek};

#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub name: String,
    pub part: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Package {
    pub pages: Vec<Page>,
    pub masters: Vec<String>,
    /// A VBA project was found. Policy strips it; CADE never runs it.
    pub has_macros: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum VisioError {
    #[error("not a Visio package: {0}")]
    NotVisio(String),
    #[error("zip: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("xml: {0}")]
    Xml(String),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

fn read_part<R: Read + Seek>(z: &mut zip::ZipArchive<R>, name: &str) -> Result<Option<String>, VisioError> {
    match z.by_name(name) {
        Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(Some(s))
        }
        Err(zip::result::ZipError::FileNotFound) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Collect `NameU` (or `Name`) attributes of `<tag>` elements, plus their r:id.
fn named_elements(xml: &str, tag: &str) -> Result<Vec<(String, String)>, VisioError> {
    let mut r = Reader::from_str(xml);
    let mut out = vec![];
    loop {
        match r.read_event() {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) if e.local_name().as_ref() == tag => {
                let mut name = String::new();
                let mut name_u = String::new();
                let mut rel = String::new();
                for a in e.attributes().flatten() {
                    let v = a
                        .normalized_value(quick_xml::XmlVersion::Implicit1_0)
                        .map_err(|x| VisioError::Xml(x.to_string()))?
                        .to_string();
                    match a.key.local_name().as_ref() {
                        "NameU" => name_u = v,
                        "Name" => name = v,
                        _ => {}
                    }
                    if a.key.as_ref() == "r:id" {
                        rel = a
                            .normalized_value(quick_xml::XmlVersion::Implicit1_0)
                            .map_err(|x| VisioError::Xml(x.to_string()))?
                            .to_string();
                    }
                }
                out.push((if name_u.is_empty() { name } else { name_u }, rel));
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(VisioError::Xml(e.to_string())),
            _ => {}
        }
    }
    Ok(out)
}

pub fn read_package<R: Read + Seek>(reader: R) -> Result<Package, VisioError> {
    let mut z = zip::ZipArchive::new(reader)?;
    let pages_xml =
        read_part(&mut z, "visio/pages/pages.xml")?.ok_or_else(|| VisioError::NotVisio("missing visio/pages/pages.xml".into()))?;
    let pages = named_elements(&pages_xml, "Page")?
        .into_iter()
        .enumerate()
        .map(|(i, (name, _))| Page {
            name,
            part: format!("visio/pages/page{}.xml", i + 1),
        })
        .collect();
    let masters = match read_part(&mut z, "visio/masters/masters.xml")? {
        Some(x) => named_elements(&x, "Master")?.into_iter().map(|(n, _)| n).collect(),
        None => vec![],
    };
    let has_macros = (0..z.len()).any(|i| {
        z.by_index(i)
            .ok()
            .and_then(|f| f.name().ok().map(|n| n.ends_with("vbaProject.bin")))
            .unwrap_or(false)
    });
    Ok(Package {
        pages,
        masters,
        has_macros,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};

    fn sample(with_macro: bool) -> Vec<u8> {
        let mut buf = Cursor::new(Vec::new());
        {
            let mut w = zip::ZipWriter::new(&mut buf);
            let o: zip::write::SimpleFileOptions = Default::default();
            w.start_file("visio/pages/pages.xml", o).unwrap();
            w.write_all(br#"<?xml version="1.0"?><Pages xmlns="http://schemas.microsoft.com/office/visio/2012/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><Page ID="0" NameU="Intake &amp; Review" Name="Intake &amp; Review"><Rel r:id="rId1"/></Page><Page ID="4" NameU="Appeals"><Rel r:id="rId2"/></Page></Pages>"#).unwrap();
            w.start_file("visio/masters/masters.xml", o).unwrap();
            w.write_all(br#"<Masters><Master ID="2" NameU="Process"/><Master ID="3" NameU="Decision"/></Masters>"#)
                .unwrap();
            if with_macro {
                w.start_file("visio/vbaProject.bin", o).unwrap();
                w.write_all(b"\0").unwrap();
            }
            w.finish().unwrap();
        }
        buf.into_inner()
    }

    #[test]
    fn reads_pages_and_masters() {
        let p = read_package(Cursor::new(sample(false))).unwrap();
        assert_eq!(
            p.pages.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(),
            vec!["Intake & Review", "Appeals"]
        );
        assert_eq!(p.masters, vec!["Process", "Decision"]);
        assert!(!p.has_macros);
    }

    #[test]
    fn detects_macros() {
        assert!(read_package(Cursor::new(sample(true))).unwrap().has_macros);
    }

    #[test]
    fn rejects_non_visio_zip() {
        let mut buf = Cursor::new(Vec::new());
        zip::ZipWriter::new(&mut buf).finish().unwrap();
        assert!(matches!(read_package(Cursor::new(buf.into_inner())), Err(VisioError::NotVisio(_))));
    }

    #[test]
    fn named_elements_name_u_present_prefers_name_u_over_name() {
        let got = named_elements(r#"<Masters><Master NameU="Process" Name="Prozess"/></Masters>"#, "Master").unwrap();
        assert_eq!(got, vec![("Process".to_string(), String::new())]);
    }

    #[test]
    fn named_elements_only_name_falls_back_to_name() {
        let got = named_elements(r#"<Masters><Master Name="Decision"/></Masters>"#, "Master").unwrap();
        assert_eq!(got, vec![("Decision".to_string(), String::new())]);
    }

    #[test]
    fn named_elements_r_id_attribute_returns_relationship_id() {
        let got = named_elements(r#"<Pages xmlns:r="urn:r"><Page NameU="Appeals" r:id="rId7"/></Pages>"#, "Page").unwrap();
        assert_eq!(got, vec![("Appeals".to_string(), "rId7".to_string())]);
    }

    #[test]
    fn named_elements_prefixed_tag_matches_by_local_name() {
        let got = named_elements(r#"<v:Pages xmlns:v="urn:v"><v:Page NameU="One"/></v:Pages>"#, "Page").unwrap();
        assert_eq!(got, vec![("One".to_string(), String::new())]);
    }

    #[test]
    fn named_elements_other_tags_are_ignored() {
        let got = named_elements(r#"<Pages><PageSheet NameU="x"/><Page NameU="A"/><Rel NameU="y"/></Pages>"#, "Page").unwrap();
        assert_eq!(got, vec![("A".to_string(), String::new())]);
    }

    #[test]
    fn named_elements_unnamed_element_returns_empty_name() {
        let got = named_elements(r#"<Pages><Page ID="0"/></Pages>"#, "Page").unwrap();
        assert_eq!(got, vec![(String::new(), String::new())]);
    }

    #[test]
    fn named_elements_mismatched_end_tag_returns_xml_error() {
        assert!(matches!(
            named_elements("<Pages><Page NameU=\"A\"></Master></Pages>", "Page"),
            Err(VisioError::Xml(_))
        ));
    }

    #[test]
    fn named_elements_bad_entity_returns_xml_error() {
        assert!(matches!(
            named_elements(r#"<Pages><Page NameU="A &bogus; B"/></Pages>"#, "Page"),
            Err(VisioError::Xml(_))
        ));
    }
}
