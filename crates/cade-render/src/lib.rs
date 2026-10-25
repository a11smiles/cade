//! The GPU canvas.
//!
//! CADE draws the canvas natively: a wgpu surface on the app window, under
//! the transparent Tauri webview that carries the panels. This crate turns
//! document nodes into a Vello scene and renders it to a texture. Hit
//! testing, selection chrome, text and images are CANVAS backlog items.

use cade_core::{Bound, Node, NodeKind, Rgba};
use vello::Scene;
use vello::kurbo::{Affine, Ellipse, Rect, RoundedRect};
use vello::peniko::{Color, Fill};

/// Canvas camera: pan in canvas px and zoom factor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Camera { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

impl Camera {
    pub fn transform(&self) -> Affine {
        Affine::scale(self.zoom) * Affine::translate((-self.x, -self.y))
    }
}

fn color(c: Rgba) -> Color {
    Color::from_rgba8(c.r, c.g, c.b, c.a)
}

/// Resolve a fill. Token fills are resolved by the caller's design system lookup.
pub fn resolve_fill(fill: &Option<Bound<Rgba>>, lookup: &dyn Fn(&str) -> Option<Rgba>) -> Option<Color> {
    match fill {
        Some(Bound::Value { value }) => Some(color(*value)),
        Some(Bound::Token { token }) => lookup(&token.name).map(color),
        None => None,
    }
}

/// Build a scene for top-level nodes. Children are drawn by the caller walking the tree.
pub fn build_scene(nodes: &[Node], camera: Camera, lookup: &dyn Fn(&str) -> Option<Rgba>) -> Scene {
    let mut scene = Scene::new();
    let base = camera.transform();
    for n in nodes.iter().filter(|n| n.visible) {
        let Some(c) = resolve_fill(&n.fill, lookup) else { continue };
        let (x, y, w, h) = (n.x as f64, n.y as f64, n.width as f64, n.height as f64);
        match &n.kind {
            NodeKind::Ellipse => scene.fill(Fill::NonZero, base, c, None, &Ellipse::from_rect(Rect::new(x, y, x + w, y + h))),
            NodeKind::Rect {
                corner_radius: Some(Bound::Value { value }),
            } => scene.fill(Fill::NonZero, base, c, None, &RoundedRect::new(x, y, x + w, y + h, *value as f64)),
            _ => scene.fill(Fill::NonZero, base, c, None, &Rect::new(x, y, x + w, y + h)),
        }
    }
    scene
}

/// GPU state for one canvas surface.
pub struct Renderer {
    pub renderer: vello::Renderer,
}

impl Renderer {
    pub fn new(device: &wgpu::Device) -> Result<Self, vello::Error> {
        let renderer = vello::Renderer::new(device, vello::RendererOptions::default())?;
        Ok(Renderer { renderer })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cade_core::TokenRef;

    #[test]
    fn builds_a_scene_with_token_fills() {
        let mut sun = Node::new("Sun", NodeKind::Ellipse);
        sun.fill = Some(Bound::Token {
            token: TokenRef {
                system: "contoso-ds".into(),
                name: "warm-300".into(),
            },
        });
        let lookup = |n: &str| (n == "warm-300").then(|| Rgba::from_hex("#F7C873").unwrap());
        let scene = build_scene(&[sun], Camera::default(), &lookup);
        assert!(!scene.encoding().is_empty());
    }
}
