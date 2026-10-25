//! Text layout for the canvas, on Parley.
//!
//! Holds the font and layout contexts and lays out a run of text at a size
//! and width. Rich text, IME, bidi, editing and font fallback configuration
//! are CANVAS backlog items.

use parley::{FontContext, Layout, LayoutContext, StyleProperty};

pub struct TextEngine {
    fonts: FontContext,
    layouts: LayoutContext<[u8; 4]>,
}

impl Default for TextEngine {
    fn default() -> Self {
        TextEngine {
            fonts: FontContext::new(),
            layouts: LayoutContext::new(),
        }
    }
}

impl TextEngine {
    /// Lay out `text` at `size_px`, wrapping at `max_width`. Returns the layout and its height.
    pub fn layout(&mut self, text: &str, size_px: f32, max_width: Option<f32>) -> (Layout<[u8; 4]>, f32) {
        let mut b = self.layouts.ranged_builder(&mut self.fonts, text, 1.0, true);
        b.push_default(StyleProperty::FontSize(size_px));
        let mut layout = b.build(text);
        layout.break_all_lines(max_width);
        let h = layout.height();
        (layout, h)
    }
}
