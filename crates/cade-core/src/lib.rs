//! Shared document types for CADE.
//!
//! These types are the vocabulary every other crate speaks: what kind of
//! output a project is, what a node on the canvas is, and how a property is
//! bound to a design-system token. Persistence lives in `cade-crdt`.
//!
//! Backlog: CORE-001 (document model), CORE-012 (components/variants),
//! CORE-014 (token bindings).

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Current on-disk format version for `.cade` projects. Bump with a migration (CORE-* format items).
pub const FORMAT_VERSION: u32 = 1;

/// The eight output types a project can be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutputKind {
    App,
    Image,
    Deck,
    Document,
    Motion,
    Diagram,
    /// Azure only in 1.0.
    Architecture,
    DataModel,
}

impl OutputKind {
    pub const ALL: [OutputKind; 8] = [
        OutputKind::App,
        OutputKind::Image,
        OutputKind::Deck,
        OutputKind::Document,
        OutputKind::Motion,
        OutputKind::Diagram,
        OutputKind::Architecture,
        OutputKind::DataModel,
    ];

    /// Uppercase label used by the type tag in the UI.
    pub fn tag(self) -> &'static str {
        match self {
            OutputKind::App => "APP",
            OutputKind::Image => "IMAGE",
            OutputKind::Deck => "DECK",
            OutputKind::Document => "DOCUMENT",
            OutputKind::Motion => "MOTION",
            OutputKind::Diagram => "DIAGRAM",
            OutputKind::Architecture => "ARCHITECTURE",
            OutputKind::DataModel => "DATA MODEL",
        }
    }
}

/// Stable id for any node. UUIDv7 so ids sort by creation time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NodeId(pub Uuid);

impl NodeId {
    pub fn new() -> Self {
        NodeId(Uuid::now_v7())
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// An sRGB color with alpha, 0–255 per channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    /// Parse `#rgb`, `#rrggbb` or `#rrggbbaa`.
    pub fn from_hex(s: &str) -> Result<Self, CoreError> {
        let h = s.strip_prefix('#').ok_or_else(|| CoreError::BadColor(s.into()))?;
        let p = |i: usize, len: usize| -> Result<u8, CoreError> {
            let part = &h[i..i + len];
            let v = u8::from_str_radix(part, 16).map_err(|_| CoreError::BadColor(s.into()))?;
            Ok(if len == 1 { v * 17 } else { v })
        };
        match h.len() {
            3 => Ok(Rgba {
                r: p(0, 1)?,
                g: p(1, 1)?,
                b: p(2, 1)?,
                a: 255,
            }),
            6 => Ok(Rgba {
                r: p(0, 2)?,
                g: p(2, 2)?,
                b: p(4, 2)?,
                a: 255,
            }),
            8 => Ok(Rgba {
                r: p(0, 2)?,
                g: p(2, 2)?,
                b: p(4, 2)?,
                a: p(6, 2)?,
            }),
            _ => Err(CoreError::BadColor(s.into())),
        }
    }

    pub fn to_hex(self) -> String {
        if self.a == 255 {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
        }
    }
}

/// A reference to a design-system token, e.g. `color.action.primary`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenRef {
    /// Design system namespace, e.g. `contoso-ds`.
    pub system: String,
    /// Token name inside that system.
    pub name: String,
}

/// A value that is either literal or bound to a token. Copilot and the
/// properties strip prefer tokens; raw values are flagged by DS lint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Bound<T> {
    Value { value: T },
    Token { token: TokenRef },
}

/// What a node on the canvas is.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum NodeKind {
    Frame,
    Group,
    Rect {
        corner_radius: Option<Bound<f32>>,
    },
    Ellipse,
    Vector {
        path: String,
    },
    Text {
        content: String,
    },
    Image {
        asset: String,
    },
    /// Main component definition.
    Component {
        variants: Vec<VariantProp>,
    },
    /// Use of a component; overrides are keyed by the inner node's id.
    Instance {
        component: NodeId,
        overrides: Vec<Override>,
    },
    /// Diagram or architecture connection between two nodes.
    Connector {
        from: NodeId,
        to: NodeId,
    },
    /// Azure resource in an Architecture project, e.g. `Microsoft.Web/sites`.
    AzureResource {
        resource_type: String,
        api_version: String,
    },
    /// Entity in a Data model project.
    Entity {
        schema: String,
        table: String,
    },
    Slide,
    Page,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariantProp {
    pub name: String,
    pub options: Vec<String>,
    pub default: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Override {
    pub target: NodeId,
    pub property: String,
    pub value: serde_json::Value,
}

/// A node with its common properties. Position and size are in canvas px.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub name: String,
    pub kind: NodeKind,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fill: Option<Bound<Rgba>>,
    pub visible: bool,
    pub locked: bool,
}

impl Node {
    pub fn new(name: impl Into<String>, kind: NodeKind) -> Self {
        Node {
            id: NodeId::new(),
            name: name.into(),
            kind,
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            fill: None,
            visible: true,
            locked: false,
        }
    }
}

/// Top-level project metadata stored next to the document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub format_version: u32,
    pub kind: OutputKind,
    pub title: String,
    /// Design systems the project is allowed to use, e.g. `contoso-ds@4`.
    pub design_systems: Vec<String>,
    /// Other projects this one is linked to in the System view.
    pub links: Vec<String>,
}

impl ProjectManifest {
    pub fn new(kind: OutputKind, title: impl Into<String>) -> Self {
        ProjectManifest {
            format_version: FORMAT_VERSION,
            kind,
            title: title.into(),
            design_systems: vec![],
            links: vec![],
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("not a color: {0}")]
    BadColor(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_round_trip() {
        assert_eq!(Rgba::from_hex("#0F6E6E").unwrap().to_hex(), "#0f6e6e");
        assert_eq!(
            Rgba::from_hex("#fff").unwrap(),
            Rgba {
                r: 255,
                g: 255,
                b: 255,
                a: 255
            }
        );
        assert_eq!(Rgba::from_hex("#00000080").unwrap().a, 128);
        assert!(Rgba::from_hex("0f6e6e").is_err());
    }

    #[test]
    fn node_serializes_with_token_fill() {
        let mut n = Node::new("Button / Primary", NodeKind::Rect { corner_radius: None });
        n.fill = Some(Bound::Token {
            token: TokenRef {
                system: "contoso-ds".into(),
                name: "color.action.primary".into(),
            },
        });
        let json = serde_json::to_string(&n).unwrap();
        assert!(json.contains("color.action.primary"));
        let back: Node = serde_json::from_str(&json).unwrap();
        assert_eq!(back, n);
    }

    #[test]
    fn every_kind_has_a_tag() {
        for k in OutputKind::ALL {
            assert!(!k.tag().is_empty());
        }
    }
}
