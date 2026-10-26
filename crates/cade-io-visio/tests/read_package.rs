//! `read_package` against synthetic Visio packages built in memory.

use cade_io_visio::{Page, VisioError, read_package};
use std::io::{Cursor, Write};

const PAGES: &str = r#"<?xml version="1.0"?><Pages xmlns="http://schemas.microsoft.com/office/visio/2012/main"><Page ID="0" NameU="Intake"/><Page ID="1" NameU="Review"/><Page ID="2" NameU="Appeals"/></Pages>"#;

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
fn read_package_three_pages_numbers_page_parts_in_order() {
    let p = read_package(Cursor::new(package(&[("visio/pages/pages.xml", PAGES.as_bytes())]))).unwrap();
    assert_eq!(
        p.pages,
        vec![
            Page {
                name: "Intake".into(),
                part: "visio/pages/page1.xml".into()
            },
            Page {
                name: "Review".into(),
                part: "visio/pages/page2.xml".into()
            },
            Page {
                name: "Appeals".into(),
                part: "visio/pages/page3.xml".into()
            },
        ]
    );
}

#[test]
fn read_package_no_masters_part_returns_no_masters() {
    let p = read_package(Cursor::new(package(&[("visio/pages/pages.xml", PAGES.as_bytes())]))).unwrap();
    assert!(p.masters.is_empty());
    assert!(!p.has_macros);
}

#[test]
fn read_package_vsdm_macro_in_any_folder_sets_has_macros() {
    let bytes = package(&[("visio/pages/pages.xml", PAGES.as_bytes()), ("custom/vbaProject.bin", b"\0\x01")]);
    assert!(read_package(Cursor::new(bytes)).unwrap().has_macros);
}

#[test]
fn read_package_file_named_like_macro_prefix_is_not_a_macro() {
    let bytes = package(&[("visio/pages/pages.xml", PAGES.as_bytes()), ("visio/vbaProject.bin.txt", b"notes")]);
    assert!(!read_package(Cursor::new(bytes)).unwrap().has_macros);
}

#[test]
fn read_package_path_traversal_entry_is_never_read_and_package_still_parses() {
    let bytes = package(&[
        ("visio/pages/pages.xml", PAGES.as_bytes()),
        ("../../etc/evil.xml", b"<evil/>"),
        ("/abs/evil.xml", b"<evil/>"),
    ]);
    let p = read_package(Cursor::new(bytes)).unwrap();
    assert_eq!(p.pages.len(), 3);
    assert!(p.masters.is_empty());
}

#[test]
fn read_package_not_a_zip_returns_zip_error() {
    let r = read_package(Cursor::new(b"%PDF-1.7 definitely not a zip".to_vec()));
    assert!(matches!(r, Err(VisioError::Zip(_))));
}

#[test]
fn read_package_empty_input_returns_zip_error() {
    assert!(matches!(read_package(Cursor::new(Vec::new())), Err(VisioError::Zip(_))));
}

#[test]
fn read_package_office_zip_without_pages_names_missing_part() {
    let bytes = package(&[("word/document.xml", b"<w:document/>")]);
    match read_package(Cursor::new(bytes)) {
        Err(VisioError::NotVisio(msg)) => assert_eq!(msg, "missing visio/pages/pages.xml"),
        other => panic!("expected NotVisio, got {other:?}"),
    }
}

#[test]
fn read_package_malformed_pages_xml_returns_xml_error() {
    let bytes = package(&[("visio/pages/pages.xml", b"<Pages><Page NameU=\"A\"></Pages>")]);
    assert!(matches!(read_package(Cursor::new(bytes)), Err(VisioError::Xml(_))));
}

#[test]
fn read_package_malformed_masters_xml_returns_xml_error() {
    let bytes = package(&[
        ("visio/pages/pages.xml", PAGES.as_bytes()),
        ("visio/masters/masters.xml", b"<Masters><Master NameU=\"A\"></Pages>"),
    ]);
    assert!(matches!(read_package(Cursor::new(bytes)), Err(VisioError::Xml(_))));
}

#[test]
fn read_package_pages_xml_not_utf8_returns_io_error() {
    let bytes = package(&[("visio/pages/pages.xml", &[0xff, 0xfe, 0x00, 0x3c])]);
    assert!(matches!(read_package(Cursor::new(bytes)), Err(VisioError::Io(_))));
}

#[test]
fn visio_error_not_visio_message_names_reason() {
    assert_eq!(
        VisioError::NotVisio("missing visio/pages/pages.xml".into()).to_string(),
        "not a Visio package: missing visio/pages/pages.xml"
    );
}
