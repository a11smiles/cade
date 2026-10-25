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

    fn no_tokens(_: &str) -> Option<Rgba> {
        None
    }

    fn filled(kind: NodeKind, hex: &str) -> Node {
        let mut n = Node::new("n", kind);
        n.fill = Some(Bound::Value {
            value: Rgba::from_hex(hex).unwrap(),
        });
        n
    }

    #[test]
    fn camera_default_is_identity() {
        assert_eq!(Camera::default(), Camera { x: 0.0, y: 0.0, zoom: 1.0 });
        assert_eq!(Camera::default().transform(), Affine::IDENTITY);
    }

    #[test]
    fn transform_pan_and_zoom_maps_camera_origin_to_screen_origin() {
        let cam = Camera {
            x: 10.0,
            y: 20.0,
            zoom: 2.0,
        };
        let t = cam.transform();
        assert_eq!(t * vello::kurbo::Point::new(10.0, 20.0), vello::kurbo::Point::new(0.0, 0.0));
        assert_eq!(t * vello::kurbo::Point::new(15.0, 21.0), vello::kurbo::Point::new(10.0, 2.0));
    }

    #[test]
    fn resolve_fill_literal_value_returns_that_color() {
        let fill = Some(Bound::Value {
            value: Rgba {
                r: 107,
                g: 87,
                b: 240,
                a: 128,
            },
        });
        let c = resolve_fill(&fill, &no_tokens).unwrap();
        assert_eq!(c.to_rgba8(), Color::from_rgba8(107, 87, 240, 128).to_rgba8());
    }

    #[test]
    fn resolve_fill_token_looks_up_token_name_only() {
        let fill = Some(Bound::Token {
            token: TokenRef {
                system: "contoso-ds".into(),
                name: "color.action.primary".into(),
            },
        });
        let seen = std::cell::RefCell::new(vec![]);
        let lookup = |n: &str| {
            seen.borrow_mut().push(n.to_string());
            Some(Rgba { r: 1, g: 2, b: 3, a: 255 })
        };
        let c = resolve_fill(&fill, &lookup).unwrap();
        assert_eq!(c.to_rgba8(), Color::from_rgba8(1, 2, 3, 255).to_rgba8());
        assert_eq!(*seen.borrow(), ["color.action.primary"]);
    }

    #[test]
    fn resolve_fill_unknown_token_returns_none() {
        let fill = Some(Bound::Token {
            token: TokenRef {
                system: "contoso-ds".into(),
                name: "missing".into(),
            },
        });
        assert!(resolve_fill(&fill, &no_tokens).is_none());
    }

    #[test]
    fn resolve_fill_no_fill_returns_none() {
        assert!(resolve_fill(&None, &|_| Some(Rgba { r: 0, g: 0, b: 0, a: 255 })).is_none());
    }

    #[test]
    fn build_scene_no_nodes_returns_empty_scene() {
        assert!(build_scene(&[], Camera::default(), &no_tokens).encoding().is_empty());
    }

    #[test]
    fn build_scene_hidden_node_is_not_drawn() {
        let mut n = filled(NodeKind::Frame, "#ffffff");
        n.visible = false;
        assert!(build_scene(&[n], Camera::default(), &no_tokens).encoding().is_empty());
    }

    #[test]
    fn build_scene_unfilled_or_unresolved_node_is_not_drawn() {
        let unfilled = Node::new("Group", NodeKind::Group);
        let mut unresolved = Node::new("Token", NodeKind::Ellipse);
        unresolved.fill = Some(Bound::Token {
            token: TokenRef {
                system: "ds".into(),
                name: "missing".into(),
            },
        });
        assert!(
            build_scene(&[unfilled, unresolved], Camera::default(), &no_tokens)
                .encoding()
                .is_empty()
        );
    }

    #[test]
    fn build_scene_visible_filled_nodes_draw_one_path_each() {
        let nodes = [
            filled(NodeKind::Frame, "#111111"),
            filled(NodeKind::Ellipse, "#222222"),
            filled(
                NodeKind::Rect {
                    corner_radius: Some(Bound::Value { value: 8.0 }),
                },
                "#333333",
            ),
        ];
        assert_eq!(build_scene(&nodes, Camera::default(), &no_tokens).encoding().n_paths, 3);
    }

    #[test]
    fn build_scene_rounded_rect_and_ellipse_encode_curves_unlike_plain_rect() {
        let tags = |kind: NodeKind| -> Vec<u8> {
            let scene = build_scene(&[filled(kind, "#000000")], Camera::default(), &no_tokens);
            scene.encoding().path_tags.iter().map(|t| t.0).collect()
        };
        let plain = tags(NodeKind::Rect { corner_radius: None });
        let rounded = tags(NodeKind::Rect {
            corner_radius: Some(Bound::Value { value: 8.0 }),
        });
        let ellipse = tags(NodeKind::Ellipse);
        assert_eq!(plain, tags(NodeKind::Frame));
        assert!(rounded.len() > plain.len(), "rounded {} vs plain {}", rounded.len(), plain.len());
        assert_ne!(ellipse, plain);
    }
}
