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
        // ASCII hex digits only: keeps the byte slicing below on char boundaries,
        // and rejects the `+` sign `from_str_radix` would otherwise accept.
        if !h.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(CoreError::BadColor(s.into()));
        }
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

    #[test]
    fn tag_each_output_kind_returns_its_ui_label() {
        let tags: Vec<&str> = OutputKind::ALL.iter().map(|k| k.tag()).collect();
        assert_eq!(
            tags,
            [
                "APP",
                "IMAGE",
                "DECK",
                "DOCUMENT",
                "MOTION",
                "DIAGRAM",
                "ARCHITECTURE",
                "DATA MODEL"
            ]
        );
    }

    #[test]
    fn output_kind_serialize_multi_word_kind_uses_kebab_case() {
        assert_eq!(serde_json::to_string(&OutputKind::DataModel).unwrap(), "\"data-model\"");
        assert_eq!(
            serde_json::from_str::<OutputKind>("\"architecture\"").unwrap(),
            OutputKind::Architecture
        );
    }

    #[test]
    fn node_id_new_each_call_returns_distinct_v7_id() {
        let (a, b) = (NodeId::new(), NodeId::new());
        assert_ne!(a, b);
        assert_eq!(a.0.get_version_num(), 7);
    }

    #[test]
    fn node_id_default_returns_v7_id() {
        assert_eq!(NodeId::default().0.get_version_num(), 7);
    }

    #[test]
    fn node_id_fmt_display_matches_hyphenated_uuid() {
        let id = NodeId(Uuid::parse_str("0192f3a4-5b6c-7d8e-9f01-23456789abcd").unwrap());
        assert_eq!(id.to_string(), "0192f3a4-5b6c-7d8e-9f01-23456789abcd");
    }

    #[test]
    fn node_id_serialize_transparent_writes_bare_uuid_string() {
        let id = NodeId(Uuid::parse_str("0192f3a4-5b6c-7d8e-9f01-23456789abcd").unwrap());
        assert_eq!(serde_json::to_string(&id).unwrap(), "\"0192f3a4-5b6c-7d8e-9f01-23456789abcd\"");
    }

    #[test]
    fn from_hex_short_form_repeats_each_digit() {
        assert_eq!(
            Rgba::from_hex("#1a3").unwrap(),
            Rgba {
                r: 0x11,
                g: 0xaa,
                b: 0x33,
                a: 255
            }
        );
    }

    #[test]
    fn from_hex_eight_digits_reads_each_channel() {
        assert_eq!(
            Rgba::from_hex("#0A141E28").unwrap(),
            Rgba {
                r: 10,
                g: 20,
                b: 30,
                a: 40
            }
        );
    }

    #[test]
    fn from_hex_unsupported_length_returns_bad_color() {
        for s in ["#", "#12", "#1234", "#12345", "#1234567", "#123456789"] {
            assert!(matches!(Rgba::from_hex(s), Err(CoreError::BadColor(v)) if v == s), "{s}");
        }
    }

    #[test]
    fn from_hex_non_hex_digit_returns_bad_color() {
        assert!(matches!(Rgba::from_hex("#gg0000"), Err(CoreError::BadColor(v)) if v == "#gg0000"));
        assert!(matches!(Rgba::from_hex("#00z"), Err(CoreError::BadColor(_))));
    }

    #[test]
    fn from_hex_non_ascii_input_returns_bad_color() {
        for s in ["#aé", "#ééé", "#0fé6e", "#\u{1F600}0"] {
            assert!(matches!(Rgba::from_hex(s), Err(CoreError::BadColor(v)) if v == s), "{s}");
        }
    }

    #[test]
    fn from_hex_sign_character_returns_bad_color() {
        for s in ["#+f+f+f", "#-1-1-1", "#+ff", "#+0+0+0+0"] {
            assert!(matches!(Rgba::from_hex(s), Err(CoreError::BadColor(v)) if v == s), "{s}");
        }
    }

    #[test]
    fn to_hex_opaque_color_omits_alpha() {
        assert_eq!(Rgba { r: 1, g: 2, b: 3, a: 255 }.to_hex(), "#010203");
    }

    #[test]
    fn to_hex_translucent_color_appends_alpha() {
        assert_eq!(Rgba { r: 255, g: 0, b: 16, a: 0 }.to_hex(), "#ff001000");
    }

    #[test]
    fn bound_serialize_value_and_token_use_kind_tag() {
        let v: Bound<f32> = Bound::Value { value: 4.0 };
        assert_eq!(
            serde_json::to_value(&v).unwrap(),
            serde_json::json!({ "kind": "value", "value": 4.0 })
        );
        let t: Bound<f32> = Bound::Token {
            token: TokenRef {
                system: "contoso-ds".into(),
                name: "radius.md".into(),
            },
        };
        assert_eq!(
            serde_json::to_value(&t).unwrap(),
            serde_json::json!({ "kind": "token", "token": { "system": "contoso-ds", "name": "radius.md" } })
        );
    }

    #[test]
    fn node_kind_serialize_variants_use_kebab_case_type_tag() {
        let azure = NodeKind::AzureResource {
            resource_type: "Microsoft.Web/sites".into(),
            api_version: "2023-12-01".into(),
        };
        assert_eq!(
            serde_json::to_value(&azure).unwrap(),
            serde_json::json!({ "type": "azure-resource", "resource_type": "Microsoft.Web/sites", "api_version": "2023-12-01" })
        );
        assert_eq!(
            serde_json::to_value(NodeKind::Frame).unwrap(),
            serde_json::json!({ "type": "frame" })
        );
        let back: NodeKind = serde_json::from_value(serde_json::json!({ "type": "rect", "corner_radius": null })).unwrap();
        assert_eq!(back, NodeKind::Rect { corner_radius: None });
    }

    #[test]
    fn node_new_sets_documented_defaults() {
        let n = Node::new("Card", NodeKind::Frame);
        assert_eq!(n.name, "Card");
        assert_eq!(n.kind, NodeKind::Frame);
        assert_eq!((n.x, n.y, n.width, n.height), (0.0, 0.0, 100.0, 100.0));
        assert_eq!(n.fill, None);
        assert!(n.visible);
        assert!(!n.locked);
        assert_eq!(n.id.0.get_version_num(), 7);
    }

    #[test]
    fn project_manifest_new_uses_current_format_and_empty_lists() {
        let m = ProjectManifest::new(OutputKind::Deck, "Q3 review");
        assert_eq!(
            m,
            ProjectManifest {
                format_version: FORMAT_VERSION,
                kind: OutputKind::Deck,
                title: "Q3 review".into(),
                design_systems: vec![],
                links: vec![],
            }
        );
    }

    #[test]
    fn core_error_fmt_bad_color_names_the_input() {
        assert_eq!(CoreError::BadColor("#12".into()).to_string(), "not a color: #12");
    }
}
