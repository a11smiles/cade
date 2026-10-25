//! Integration tests for `cade_crdt::Project`: the layer tree, snapshots,
//! merges between replicas and the JSON mirror.

use cade_core::{Bound, FORMAT_VERSION, Node, NodeId, NodeKind, OutputKind, ProjectManifest, Rgba};
use cade_crdt::{CrdtError, Project};

fn names(nodes: Vec<Node>) -> Vec<String> {
    nodes.into_iter().map(|n| n.name).collect()
}

#[test]
fn project_new_manifest_records_kind_title_and_format_version() {
    let p = Project::new(OutputKind::Architecture, "Claims platform").unwrap();
    assert_eq!(
        p.manifest().unwrap(),
        ProjectManifest::new(OutputKind::Architecture, "Claims platform")
    );
    assert_eq!(p.manifest().unwrap().format_version, FORMAT_VERSION);
    assert!(p.children(None).unwrap().is_empty());
}

#[test]
fn project_insert_unknown_parent_returns_no_node() {
    let mut p = Project::new(OutputKind::App, "x").unwrap();
    let ghost = NodeId::new();
    let err = p.insert(Some(ghost), Node::new("Orphan", NodeKind::Frame)).unwrap_err();
    assert!(matches!(err, CrdtError::NoNode(id) if id == ghost));
    assert!(p.children(None).unwrap().is_empty());
}

#[test]
fn project_get_inserted_node_returns_every_field() {
    let mut p = Project::new(OutputKind::App, "x").unwrap();
    let mut n = Node::new(
        "Button / Primary",
        NodeKind::Rect {
            corner_radius: Some(Bound::Value { value: 6.0 }),
        },
    );
    n.x = 12.5;
    n.y = -4.0;
    n.width = 120.0;
    n.height = 40.0;
    n.fill = Some(Bound::Value {
        value: Rgba::from_hex("#6b57f0").unwrap(),
    });
    n.locked = true;
    let id = p.insert(None, n.clone()).unwrap();
    assert_eq!(id, n.id);
    assert_eq!(p.get(id).unwrap(), n);
}

#[test]
fn project_get_unknown_id_returns_no_node() {
    let p = Project::new(OutputKind::App, "x").unwrap();
    let ghost = NodeId::new();
    assert!(matches!(p.get(ghost), Err(CrdtError::NoNode(id)) if id == ghost));
}

#[test]
fn project_update_existing_node_replaces_stored_node() {
    let mut p = Project::new(OutputKind::App, "x").unwrap();
    let mut n = Node::new("Header", NodeKind::Frame);
    p.insert(None, n.clone()).unwrap();
    n.name = "Header / Sticky".into();
    n.visible = false;
    p.update(&n).unwrap();
    assert_eq!(p.get(n.id).unwrap(), n);
}

#[test]
fn project_update_unknown_node_returns_no_node() {
    let mut p = Project::new(OutputKind::App, "x").unwrap();
    let n = Node::new("Never inserted", NodeKind::Frame);
    assert!(matches!(p.update(&n), Err(CrdtError::NoNode(id)) if id == n.id));
}

#[test]
fn project_children_root_lists_layers_in_insertion_order() {
    let mut p = Project::new(OutputKind::Deck, "x").unwrap();
    for name in ["Slide 1", "Slide 2", "Slide 3"] {
        p.insert(None, Node::new(name, NodeKind::Slide)).unwrap();
    }
    assert_eq!(names(p.children(None).unwrap()), ["Slide 1", "Slide 2", "Slide 3"]);
}

#[test]
fn project_children_unknown_parent_returns_no_node() {
    let p = Project::new(OutputKind::App, "x").unwrap();
    let ghost = NodeId::new();
    assert!(matches!(p.children(Some(ghost)), Err(CrdtError::NoNode(id)) if id == ghost));
}

#[test]
fn project_reparent_to_none_moves_layer_to_root() {
    let mut p = Project::new(OutputKind::App, "x").unwrap();
    let frame = p.insert(None, Node::new("Frame", NodeKind::Frame)).unwrap();
    let child = p.insert(Some(frame), Node::new("Child", NodeKind::Ellipse)).unwrap();
    p.reparent(child, None).unwrap();
    assert!(p.children(Some(frame)).unwrap().is_empty());
    assert_eq!(names(p.children(None).unwrap()), ["Frame", "Child"]);
}

#[test]
fn project_reparent_unknown_node_or_parent_returns_no_node() {
    let mut p = Project::new(OutputKind::App, "x").unwrap();
    let a = p.insert(None, Node::new("A", NodeKind::Frame)).unwrap();
    let ghost = NodeId::new();
    assert!(matches!(p.reparent(ghost, None), Err(CrdtError::NoNode(id)) if id == ghost));
    assert!(matches!(p.reparent(a, Some(ghost)), Err(CrdtError::NoNode(id)) if id == ghost));
    assert_eq!(names(p.children(None).unwrap()), ["A"]);
}

#[test]
fn project_reparent_into_own_descendant_returns_loro_error_and_keeps_tree() {
    let mut p = Project::new(OutputKind::App, "x").unwrap();
    let a = p.insert(None, Node::new("A", NodeKind::Frame)).unwrap();
    let b = p.insert(Some(a), Node::new("B", NodeKind::Frame)).unwrap();
    assert!(matches!(p.reparent(a, Some(b)), Err(CrdtError::Loro(_))));
    assert_eq!(names(p.children(None).unwrap()), ["A"]);
    assert_eq!(names(p.children(Some(a)).unwrap()), ["B"]);
}

#[test]
fn project_open_snapshot_restores_index_for_get_and_update() {
    let mut p = Project::new(OutputKind::App, "Claims Portal").unwrap();
    let mut n = Node::new("Step 1", NodeKind::Frame);
    p.insert(None, n.clone()).unwrap();
    let mut q = Project::open(&p.snapshot().unwrap()).unwrap();
    assert_eq!(q.get(n.id).unwrap(), n);
    n.name = "Step 1 · Mobile".into();
    q.update(&n).unwrap();
    assert_eq!(q.get(n.id).unwrap().name, "Step 1 · Mobile");
}

#[test]
fn project_open_snapshot_round_trip_preserves_json_mirror() {
    let mut p = Project::new(OutputKind::Diagram, "Intake").unwrap();
    let lane = p.insert(None, Node::new("Lane", NodeKind::Frame)).unwrap();
    p.insert(Some(lane), Node::new("Start", NodeKind::Ellipse)).unwrap();
    let q = Project::open(&p.snapshot().unwrap()).unwrap();
    assert_eq!(q.to_json().unwrap(), p.to_json().unwrap());
}

#[test]
fn project_open_garbage_bytes_returns_loro_error() {
    assert!(matches!(Project::open(b"not a snapshot"), Err(CrdtError::Loro(_))));
}

#[test]
fn project_open_loro_doc_without_manifest_returns_not_a_project() {
    let doc = loro::LoroDoc::new();
    doc.get_map("other").insert("k", "v").unwrap();
    doc.commit();
    let bytes = doc.export(loro::ExportMode::Snapshot).unwrap();
    assert!(matches!(Project::open(&bytes), Err(CrdtError::NotAProject)));
}

#[test]
fn project_merge_garbage_bytes_returns_loro_error() {
    let mut p = Project::new(OutputKind::App, "x").unwrap();
    assert!(matches!(p.merge(b"\x00\x01garbage"), Err(CrdtError::Loro(_))));
}

#[test]
fn project_merge_indexes_nodes_created_by_other_replica() {
    let mut a = Project::new(OutputKind::App, "x").unwrap();
    let mut b = Project::open(&a.snapshot().unwrap()).unwrap();
    let from_b = b.insert(None, Node::new("From B", NodeKind::Frame)).unwrap();
    a.merge(&b.snapshot().unwrap()).unwrap();
    assert_eq!(a.get(from_b).unwrap().name, "From B");
    a.insert(Some(from_b), Node::new("Child from A", NodeKind::Ellipse)).unwrap();
    assert_eq!(names(a.children(Some(from_b)).unwrap()), ["Child from A"]);
}

#[test]
fn project_merge_both_directions_converges_to_same_document() {
    let mut a = Project::new(OutputKind::Image, "Banner").unwrap();
    let root = a.insert(None, Node::new("LinkedIn", NodeKind::Frame)).unwrap();
    let mut b = Project::open(&a.snapshot().unwrap()).unwrap();
    a.insert(Some(root), Node::new("Sun", NodeKind::Ellipse)).unwrap();
    b.insert(Some(root), Node::new("Logo", NodeKind::Image { asset: "logo.svg".into() }))
        .unwrap();
    let (sa, sb) = (a.snapshot().unwrap(), b.snapshot().unwrap());
    a.merge(&sb).unwrap();
    b.merge(&sa).unwrap();
    assert_eq!(a.to_json().unwrap(), b.to_json().unwrap());
    assert_eq!(a.children(Some(root)).unwrap().len(), 2);
}

#[test]
fn project_merge_concurrent_updates_to_one_node_converge() {
    let mut a = Project::new(OutputKind::App, "x").unwrap();
    let mut n = Node::new("Title", NodeKind::Text { content: "Hi".into() });
    a.insert(None, n.clone()).unwrap();
    let mut b = Project::open(&a.snapshot().unwrap()).unwrap();
    n.name = "Title A".into();
    a.update(&n).unwrap();
    n.name = "Title B".into();
    b.update(&n).unwrap();
    let (sa, sb) = (a.snapshot().unwrap(), b.snapshot().unwrap());
    a.merge(&sb).unwrap();
    b.merge(&sa).unwrap();
    let (na, nb) = (a.get(n.id).unwrap().name, b.get(n.id).unwrap().name);
    assert_eq!(na, nb);
    assert!(na == "Title A" || na == "Title B", "{na}");
}

#[test]
fn project_merge_concurrent_cross_moves_leaves_no_cycle() {
    let mut a = Project::new(OutputKind::App, "x").unwrap();
    let x = a.insert(None, Node::new("X", NodeKind::Frame)).unwrap();
    let y = a.insert(None, Node::new("Y", NodeKind::Frame)).unwrap();
    let mut b = Project::open(&a.snapshot().unwrap()).unwrap();
    a.reparent(x, Some(y)).unwrap();
    b.reparent(y, Some(x)).unwrap();
    let (sa, sb) = (a.snapshot().unwrap(), b.snapshot().unwrap());
    a.merge(&sb).unwrap();
    b.merge(&sa).unwrap();
    for p in [&a, &b] {
        let roots = names(p.children(None).unwrap());
        assert_eq!(roots.len(), 1, "exactly one of X and Y stays at the root: {roots:?}");
        let top = if roots[0] == "X" { x } else { y };
        assert_eq!(p.children(Some(top)).unwrap().len(), 1);
    }
    assert_eq!(a.to_json().unwrap(), b.to_json().unwrap());
}

#[test]
fn project_to_json_nests_children_and_omits_empty_children() {
    let mut p = Project::new(OutputKind::App, "Claims Portal").unwrap();
    let frame = p.insert(None, Node::new("Step 1", NodeKind::Frame)).unwrap();
    let leaf = p.insert(Some(frame), Node::new("Header", NodeKind::Frame)).unwrap();
    let v: serde_json::Value = serde_json::from_str(&p.to_json().unwrap()).unwrap();
    assert_eq!(v["manifest"]["title"], "Claims Portal");
    assert_eq!(v["manifest"]["kind"], "app");
    let layers = v["layers"].as_array().unwrap();
    assert_eq!(layers.len(), 1);
    assert_eq!(layers[0]["id"], frame.to_string());
    assert_eq!(layers[0]["kind"]["type"], "frame");
    let kids = layers[0]["children"].as_array().unwrap();
    assert_eq!(kids.len(), 1);
    assert_eq!(kids[0]["id"], leaf.to_string());
    assert!(kids[0].get("children").is_none());
}

#[test]
fn project_to_json_empty_project_has_empty_layers() {
    let p = Project::new(OutputKind::DataModel, "Claims").unwrap();
    let v: serde_json::Value = serde_json::from_str(&p.to_json().unwrap()).unwrap();
    assert_eq!(v["layers"], serde_json::json!([]));
    assert_eq!(v["manifest"]["kind"], "data-model");
}
