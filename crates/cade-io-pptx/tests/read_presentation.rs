//! `read_presentation` against synthetic PowerPoint packages built in memory.

use cade_io_pptx::{PptxError, Presentation, read_presentation};
use std::io::{Cursor, Write};

const PRESENTATION: (&str, &[u8]) = ("ppt/presentation.xml", b"<p:presentation xmlns:p=\"urn:p\"/>");
const PPTX_TYPES: &[u8] =
    br#"<Types><Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/></Types>"#;
const POTX_TYPES: &[u8] =
    br#"<Types><Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.template.main+xml"/></Types>"#;

fn layout(name: &str) -> Vec<u8> {
    format!(r#"<p:sldLayout xmlns:p="urn:p"><p:cSld name="{name}"><p:spTree/></p:cSld></p:sldLayout>"#).into_bytes()
}

fn package(parts: &[(&str, &[u8])]) -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());
    {
        let mut w = zip::ZipWriter::new(&mut buf);
        let o: zip::write::SimpleFileOptions = Default::default();
        for (name, bytes) in parts {
            w.start_file(*name, o).unwrap();
            w.write_all(bytes).unwrap();
        }
        w.finish().unwrap();
    }
    buf.into_inner()
}

#[test]
fn read_presentation_slides_and_layouts_returns_count_and_names() {
    let (title, two) = (layout("Title Slide"), layout("Title and Content"));
    let bytes = package(&[
        ("[Content_Types].xml", PPTX_TYPES),
        PRESENTATION,
        ("ppt/slides/slide1.xml", b"<p:sld/>"),
        ("ppt/slides/slide2.xml", b"<p:sld/>"),
        ("ppt/slides/slide3.xml", b"<p:sld/>"),
        ("ppt/slideLayouts/slideLayout1.xml", &title),
        ("ppt/slideLayouts/slideLayout2.xml", &two),
    ]);
    assert_eq!(
        read_presentation(Cursor::new(bytes)).unwrap(),
        Presentation {
            slide_count: 3,
            layouts: vec!["Title Slide".into(), "Title and Content".into()],
            is_template: false,
        }
    );
}

#[test]
fn read_presentation_slide_rels_and_notes_are_not_counted_as_slides() {
    let bytes = package(&[
        PRESENTATION,
        ("ppt/slides/slide1.xml", b"<p:sld/>"),
        ("ppt/slides/_rels/slide1.xml.rels", b"<Relationships/>"),
        ("ppt/notesSlides/notesSlide1.xml", b"<p:notes/>"),
        ("ppt/slideLayouts/_rels/slideLayout1.xml.rels", b"<Relationships/>"),
    ]);
    let p = read_presentation(Cursor::new(bytes)).unwrap();
    assert_eq!(p.slide_count, 1);
    assert!(p.layouts.is_empty());
}

#[test]
fn read_presentation_potx_content_type_sets_is_template() {
    let bytes = package(&[("[Content_Types].xml", POTX_TYPES), PRESENTATION]);
    assert!(read_presentation(Cursor::new(bytes)).unwrap().is_template);
}

#[test]
fn read_presentation_no_content_types_part_is_not_template() {
    let bytes = package(&[PRESENTATION]);
    assert_eq!(read_presentation(Cursor::new(bytes)).unwrap(), Presentation::default());
}

#[test]
fn read_presentation_layout_without_name_is_skipped() {
    let named = layout("Blank");
    let bytes = package(&[
        PRESENTATION,
        (
            "ppt/slideLayouts/slideLayout1.xml",
            b"<p:sldLayout xmlns:p=\"urn:p\"><p:cSld><p:spTree/></p:cSld></p:sldLayout>",
        ),
        ("ppt/slideLayouts/slideLayout2.xml", &named),
    ]);
    assert_eq!(read_presentation(Cursor::new(bytes)).unwrap().layouts, vec!["Blank"]);
}

#[test]
fn read_presentation_layout_name_entities_are_unescaped() {
    let amp = layout("Section &amp; Header");
    let bytes = package(&[PRESENTATION, ("ppt/slideLayouts/slideLayout1.xml", &amp)]);
    assert_eq!(read_presentation(Cursor::new(bytes)).unwrap().layouts, vec!["Section & Header"]);
}

#[test]
fn read_presentation_path_traversal_entry_is_ignored() {
    let bytes = package(&[
        PRESENTATION,
        ("../../ppt/slides/slide9.xml", b"<p:sld/>"),
        ("ppt/slides/slide1.xml", b"<p:sld/>"),
    ]);
    assert_eq!(read_presentation(Cursor::new(bytes)).unwrap().slide_count, 1);
}

#[test]
fn read_presentation_word_document_returns_not_pptx() {
    let bytes = package(&[("word/document.xml", b"<w:document/>")]);
    assert!(matches!(read_presentation(Cursor::new(bytes)), Err(PptxError::NotPptx)));
}

#[test]
fn read_presentation_not_a_zip_returns_zip_error() {
    assert!(matches!(
        read_presentation(Cursor::new(b"not a zip at all".to_vec())),
        Err(PptxError::Zip(_))
    ));
}

#[test]
fn read_presentation_malformed_layout_returns_xml_error() {
    let bytes = package(&[
        PRESENTATION,
        ("ppt/slideLayouts/slideLayout1.xml", b"<p:sldLayout><p:spTree></p:sldLayout>"),
    ]);
    assert!(matches!(read_presentation(Cursor::new(bytes)), Err(PptxError::Xml(_))));
}

#[test]
fn read_presentation_layout_not_utf8_returns_io_error() {
    let bytes = package(&[PRESENTATION, ("ppt/slideLayouts/slideLayout1.xml", &[0xff, 0xfe, 0x00])]);
    assert!(matches!(read_presentation(Cursor::new(bytes)), Err(PptxError::Io(_))));
}

#[test]
fn pptx_error_not_pptx_message_is_plain() {
    assert_eq!(PptxError::NotPptx.to_string(), "not a PowerPoint package");
}
