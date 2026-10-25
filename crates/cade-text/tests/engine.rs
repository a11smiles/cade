//! Integration tests for `cade_text::TextEngine`. They use the system's fonts,
//! so they assert relationships between layouts rather than exact pixel sizes.

use cade_text::TextEngine;

const PARAGRAPH: &str = "Copilot is paused until Nov 1. Your prompt is saved. You can keep editing by hand, export and ship.";

#[test]
fn text_engine_layout_returned_height_equals_layout_height() {
    let mut e = TextEngine::default();
    let (layout, h) = e.layout(PARAGRAPH, 13.0, Some(200.0));
    assert_eq!(h, layout.height());
    assert!(h > 0.0);
}

#[test]
fn text_engine_layout_no_max_width_keeps_one_line() {
    let mut e = TextEngine::default();
    let (layout, _) = e.layout(PARAGRAPH, 13.0, None);
    assert_eq!(layout.len(), 1);
}

#[test]
fn text_engine_layout_narrow_max_width_wraps_within_width() {
    let mut e = TextEngine::default();
    let (wide, wide_h) = e.layout(PARAGRAPH, 13.0, None);
    let (narrow, narrow_h) = e.layout(PARAGRAPH, 13.0, Some(120.0));
    assert!(narrow.len() > 1, "expected wrapping, got {} line(s)", narrow.len());
    assert!(narrow_h > wide_h);
    assert!(narrow.width() <= 120.0 + f32::EPSILON, "{}", narrow.width());
    assert!(narrow.width() < wide.width());
}

#[test]
fn text_engine_layout_larger_size_produces_taller_layout() {
    let mut e = TextEngine::default();
    let (_, small) = e.layout("Apply 6 changes", 11.0, None);
    let (_, large) = e.layout("Apply 6 changes", 22.0, None);
    assert!(large > small * 1.5, "11px: {small}, 22px: {large}");
}
