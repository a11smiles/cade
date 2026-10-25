//! `.cade` project documents on the Loro CRDT.
//!
//! A project is one Loro document:
//!
//! - `manifest` (map): the serialized [`ProjectManifest`].
//! - `layers` (movable tree): one tree node per canvas node. Each node's
//!   metadata map holds the serialized [`Node`] under `node`. The movable
//!   tree gives conflict-free reparenting and reordering, which is what
//!   layer panels need when two people, or a person and Copilot, edit at once.
//!
//! On disk a project is the Loro snapshot plus a JSON mirror for readable
//! diffs in pull requests (CORE backlog: file format, git persistence).

use cade_core::{Node, NodeId, OutputKind, ProjectManifest};
use loro::{ExportMode, LoroDoc, LoroValue, TreeID, TreeParentId};
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum CrdtError {
    #[error("loro: {0}")]
    Loro(String),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("no node {0}")]
    NoNode(NodeId),
    #[error("not a CADE project")]
    NotAProject,
}

fn le<E: std::fmt::Display>(e: E) -> CrdtError {
    CrdtError::Loro(e.to_string())
}

pub struct Project {
    doc: LoroDoc,
    index: HashMap<NodeId, TreeID>,
}

impl Project {
    pub fn new(kind: OutputKind, title: &str) -> Result<Self, CrdtError> {
        let doc = LoroDoc::new();
        doc.get_tree("layers").enable_fractional_index(0);
        let manifest = serde_json::to_string(&ProjectManifest::new(kind, title))?;
        doc.get_map("manifest").insert("json", manifest).map_err(le)?;
        doc.commit();
        Ok(Project {
            doc,
            index: HashMap::new(),
        })
    }

    pub fn manifest(&self) -> Result<ProjectManifest, CrdtError> {
        match self.doc.get_map("manifest").get("json") {
            Some(v) => match v.into_value() {
                Ok(LoroValue::String(s)) => Ok(serde_json::from_str(&s)?),
                _ => Err(CrdtError::NotAProject),
            },
            None => Err(CrdtError::NotAProject),
        }
    }

    /// Add a node under `parent` (or at the root). Returns its id.
    pub fn insert(&mut self, parent: Option<NodeId>, node: Node) -> Result<NodeId, CrdtError> {
        let tree = self.doc.get_tree("layers");
        let parent_id: TreeParentId = match parent {
            Some(p) => TreeParentId::Node(*self.index.get(&p).ok_or(CrdtError::NoNode(p))?),
            None => TreeParentId::Root,
        };
        let tid = tree.create(parent_id).map_err(le)?;
        let meta = tree.get_meta(tid).map_err(le)?;
        meta.insert("node", serde_json::to_string(&node)?).map_err(le)?;
        self.index.insert(node.id, tid);
        self.doc.commit();
        Ok(node.id)
    }

    /// Reparent a node. Conflicting concurrent moves resolve without cycles.
    pub fn reparent(&mut self, id: NodeId, new_parent: Option<NodeId>) -> Result<(), CrdtError> {
        let tree = self.doc.get_tree("layers");
        let tid = *self.index.get(&id).ok_or(CrdtError::NoNode(id))?;
        let p = match new_parent {
            Some(p) => TreeParentId::Node(*self.index.get(&p).ok_or(CrdtError::NoNode(p))?),
            None => TreeParentId::Root,
        };
        tree.mov(tid, p).map_err(le)?;
        self.doc.commit();
        Ok(())
    }

    pub fn update(&mut self, node: &Node) -> Result<(), CrdtError> {
        let tid = *self.index.get(&node.id).ok_or(CrdtError::NoNode(node.id))?;
        let meta = self.doc.get_tree("layers").get_meta(tid).map_err(le)?;
        meta.insert("node", serde_json::to_string(node)?).map_err(le)?;
        self.doc.commit();
        Ok(())
    }

    pub fn get(&self, id: NodeId) -> Result<Node, CrdtError> {
        let tid = *self.index.get(&id).ok_or(CrdtError::NoNode(id))?;
        self.read(tid)
    }

    fn read(&self, tid: TreeID) -> Result<Node, CrdtError> {
        let meta = self.doc.get_tree("layers").get_meta(tid).map_err(le)?;
        match meta.get("node").map(|v| v.into_value()) {
            Some(Ok(LoroValue::String(s))) => Ok(serde_json::from_str(&s)?),
            _ => Err(CrdtError::NotAProject),
        }
    }

    /// Children of `parent` (root when `None`), in layer order.
    pub fn children(&self, parent: Option<NodeId>) -> Result<Vec<Node>, CrdtError> {
        let tree = self.doc.get_tree("layers");
        let p = match parent {
            Some(p) => TreeParentId::Node(*self.index.get(&p).ok_or(CrdtError::NoNode(p))?),
            None => TreeParentId::Root,
        };
        tree.children(p).unwrap_or_default().into_iter().map(|t| self.read(t)).collect()
    }

    /// Full Loro snapshot (history + state) for the `.cade` file.
    pub fn snapshot(&self) -> Result<Vec<u8>, CrdtError> {
        self.doc.export(ExportMode::Snapshot).map_err(le)
    }

    pub fn open(bytes: &[u8]) -> Result<Self, CrdtError> {
        let doc = LoroDoc::from_snapshot(bytes).map_err(le)?;
        let mut p = Project {
            doc,
            index: HashMap::new(),
        };
        let tree = p.doc.get_tree("layers");
        for tid in tree.nodes() {
            if let Ok(n) = p.read(tid) {
                p.index.insert(n.id, tid);
            }
        }
        p.manifest()?;
        Ok(p)
    }

    /// Merge changes from another replica (a teammate, or a Copilot branch).
    pub fn merge(&mut self, other_snapshot: &[u8]) -> Result<(), CrdtError> {
        self.doc.import(other_snapshot).map_err(le)?;
        let tree = self.doc.get_tree("layers");
        for tid in tree.nodes() {
            if let Ok(n) = self.read(tid) {
                self.index.insert(n.id, tid);
            }
        }
        Ok(())
    }

    /// Readable JSON mirror written next to the snapshot for PR diffs.
    pub fn to_json(&self) -> Result<String, CrdtError> {
        fn walk(p: &Project, parent: Option<NodeId>) -> Result<Vec<serde_json::Value>, CrdtError> {
            let mut out = vec![];
            for n in p.children(parent)? {
                let kids = walk(p, Some(n.id))?;
                let mut v = serde_json::to_value(&n)?;
                if !kids.is_empty() {
                    v["children"] = serde_json::Value::Array(kids);
                }
                out.push(v);
            }
            Ok(out)
        }
        Ok(serde_json::to_string_pretty(
            &serde_json::json!({ "manifest": self.manifest()?, "layers": walk(self, None)? }),
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cade_core::NodeKind;

    #[test]
    fn build_save_open_round_trip() {
        let mut p = Project::new(OutputKind::App, "Claims Portal").unwrap();
        let frame = p.insert(None, Node::new("Step 1 · Mobile", NodeKind::Frame)).unwrap();
        p.insert(Some(frame), Node::new("Header", NodeKind::Frame)).unwrap();
        p.insert(Some(frame), Node::new("Button / Primary", NodeKind::Rect { corner_radius: None }))
            .unwrap();
        let bytes = p.snapshot().unwrap();
        let q = Project::open(&bytes).unwrap();
        assert_eq!(q.manifest().unwrap().title, "Claims Portal");
        let kids: Vec<String> = q.children(Some(frame)).unwrap().into_iter().map(|n| n.name).collect();
        assert_eq!(kids, vec!["Header", "Button / Primary"]);
        assert!(q.to_json().unwrap().contains("\"children\""));
    }

    #[test]
    fn concurrent_edits_merge() {
        let mut a = Project::new(OutputKind::Image, "Banner").unwrap();
        let root = a.insert(None, Node::new("LinkedIn", NodeKind::Frame)).unwrap();
        let mut b = Project::open(&a.snapshot().unwrap()).unwrap();
        a.insert(Some(root), Node::new("Sun", NodeKind::Ellipse)).unwrap();
        b.insert(
            Some(root),
            Node::new(
                "Headline",
                NodeKind::Text {
                    content: "Open enrollment".into(),
                },
            ),
        )
        .unwrap();
        a.merge(&b.snapshot().unwrap()).unwrap();
        let mut names: Vec<String> = a.children(Some(root)).unwrap().into_iter().map(|n| n.name).collect();
        names.sort();
        assert_eq!(names, vec!["Headline", "Sun"]);
    }

    #[test]
    fn reparent_moves_a_layer() {
        let mut p = Project::new(OutputKind::App, "x").unwrap();
        let a = p.insert(None, Node::new("A", NodeKind::Frame)).unwrap();
        let b = p.insert(None, Node::new("B", NodeKind::Frame)).unwrap();
        let c = p.insert(Some(a), Node::new("C", NodeKind::Ellipse)).unwrap();
        p.reparent(c, Some(b)).unwrap();
        assert!(p.children(Some(a)).unwrap().is_empty());
        assert_eq!(p.children(Some(b)).unwrap()[0].id, c);
    }
}
