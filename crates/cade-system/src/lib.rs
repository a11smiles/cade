//! The linked system: Data model → App → Architecture.
//!
//! Each project exposes the things it produces and consumes. This crate
//! connects them into a graph so CADE can show bindings, trace the impact of
//! a change across layers, find broken links (like a setting the app reads
//! but the architecture never defines) and order a deploy plan.
//!
//! Backlog: SYS epic.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Layer {
    Data,
    App,
    Architecture,
}

/// A thing in one layer, e.g. `Data:Claim.status`, `App:StatusBadge`, `Architecture:sql-claims-stg`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Item {
    pub layer: Layer,
    pub key: String,
}

impl Item {
    pub fn new(layer: Layer, key: &str) -> Self {
        Item { layer, key: key.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Binding {
    pub from: Item,
    pub to: Item,
    /// Shown in the bindings table, e.g. "Runs against the database before the new app version starts".
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    InSync,
    Pending,
    /// The `to` side does not exist.
    Missing,
}

#[derive(Debug, Default)]
pub struct SystemGraph {
    items: BTreeSet<Item>,
    bindings: Vec<Binding>,
    pending: BTreeSet<Item>,
}

impl SystemGraph {
    pub fn add_item(&mut self, item: Item) {
        self.items.insert(item);
    }
    pub fn mark_pending(&mut self, item: Item) {
        self.pending.insert(item);
    }
    pub fn bind(&mut self, from: Item, to: Item, description: &str) {
        self.bindings.push(Binding {
            from,
            to,
            description: description.into(),
        });
    }

    pub fn status(&self, b: &Binding) -> Status {
        if !self.items.contains(&b.to) {
            Status::Missing
        } else if self.pending.contains(&b.from) || self.pending.contains(&b.to) {
            Status::Pending
        } else {
            Status::InSync
        }
    }

    pub fn bindings(&self) -> impl Iterator<Item = (&Binding, Status)> {
        self.bindings.iter().map(|b| (b, self.status(b)))
    }

    /// Everything downstream of `start`, in breadth-first order.
    pub fn impact(&self, start: &Item) -> Vec<Item> {
        let mut out = vec![];
        let mut seen = BTreeSet::new();
        let mut q = VecDeque::from([start.clone()]);
        while let Some(cur) = q.pop_front() {
            for b in self.bindings.iter().filter(|b| b.from == cur) {
                if seen.insert(b.to.clone()) {
                    out.push(b.to.clone());
                    q.push_back(b.to.clone());
                }
            }
        }
        out
    }

    /// Deploy steps grouped by layer in the order they must run.
    pub fn deploy_order(&self, changed: &[Item]) -> Vec<(Layer, Vec<Item>)> {
        let mut all: BTreeSet<Item> = changed.iter().cloned().collect();
        for c in changed {
            all.extend(self.impact(c));
        }
        let mut by: BTreeMap<Layer, Vec<Item>> = BTreeMap::new();
        for i in all {
            by.entry(i.layer).or_default().push(i);
        }
        // Infrastructure first, then data migrations, then the app.
        [Layer::Architecture, Layer::Data, Layer::App]
            .into_iter()
            .filter_map(|l| by.remove(&l).map(|v| (l, v)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claims() -> SystemGraph {
        let mut g = SystemGraph::default();
        let status = Item::new(Layer::Data, "Claim.status");
        let badge = Item::new(Layer::App, "StatusBadge");
        let mig = Item::new(Layer::Data, "migration 0007");
        let db = Item::new(Layer::Architecture, "sql-claims-stg");
        for i in [&status, &badge, &mig, &db] {
            g.add_item(i.clone());
        }
        g.bind(status.clone(), badge, "Enum renders as status badge");
        g.bind(status, mig.clone(), "Schema change");
        g.bind(mig.clone(), db, "Runs before the new app version starts");
        g.mark_pending(mig);
        g.bind(
            Item::new(Layer::App, "PHOTO_CONTAINER"),
            Item::new(Layer::Architecture, "setting:PHOTO_CONTAINER"),
            "App reads this setting",
        );
        g
    }

    #[test]
    fn statuses_match_the_system_view() {
        let g = claims();
        let s: Vec<Status> = g.bindings().map(|(_, s)| s).collect();
        assert_eq!(s, vec![Status::InSync, Status::Pending, Status::Pending, Status::Missing]);
    }

    #[test]
    fn traces_impact_across_layers() {
        let g = claims();
        let hit = g.impact(&Item::new(Layer::Data, "Claim.status"));
        assert!(hit.contains(&Item::new(Layer::App, "StatusBadge")));
        assert!(hit.contains(&Item::new(Layer::Architecture, "sql-claims-stg")));
    }

    #[test]
    fn deploys_infra_then_data_then_app() {
        let g = claims();
        let plan = g.deploy_order(&[Item::new(Layer::Data, "Claim.status")]);
        let layers: Vec<Layer> = plan.iter().map(|(l, _)| *l).collect();
        assert_eq!(layers, vec![Layer::Architecture, Layer::Data, Layer::App]);
    }

    fn item(layer: Layer, key: &str) -> Item {
        Item::new(layer, key)
    }

    #[test]
    fn item_new_copies_layer_and_key_returns_item() {
        assert_eq!(
            Item::new(Layer::App, "StatusBadge"),
            Item {
                layer: Layer::App,
                key: "StatusBadge".to_string()
            }
        );
    }

    #[test]
    fn status_target_missing_and_pending_returns_missing() {
        let mut g = SystemGraph::default();
        let from = item(Layer::App, "PHOTO_CONTAINER");
        g.add_item(from.clone());
        g.mark_pending(from.clone());
        let b = Binding {
            from,
            to: item(Layer::Architecture, "setting:PHOTO_CONTAINER"),
            description: String::new(),
        };
        assert_eq!(g.status(&b), Status::Missing);
    }

    #[test]
    fn status_source_pending_returns_pending() {
        let mut g = SystemGraph::default();
        let (from, to) = (item(Layer::Data, "migration 0007"), item(Layer::Architecture, "sql-claims-stg"));
        g.add_item(from.clone());
        g.add_item(to.clone());
        g.mark_pending(from.clone());
        let b = Binding {
            from,
            to,
            description: String::new(),
        };
        assert_eq!(g.status(&b), Status::Pending);
    }

    #[test]
    fn status_target_pending_returns_pending() {
        let mut g = SystemGraph::default();
        let (from, to) = (item(Layer::Data, "Claim.status"), item(Layer::Data, "migration 0007"));
        g.add_item(to.clone());
        g.mark_pending(to.clone());
        let b = Binding {
            from,
            to,
            description: String::new(),
        };
        assert_eq!(g.status(&b), Status::Pending);
    }

    #[test]
    fn status_source_not_registered_target_present_returns_in_sync() {
        let mut g = SystemGraph::default();
        let to = item(Layer::App, "StatusBadge");
        g.add_item(to.clone());
        let b = Binding {
            from: item(Layer::Data, "Claim.status"),
            to,
            description: String::new(),
        };
        assert_eq!(g.status(&b), Status::InSync);
    }

    #[test]
    fn bindings_several_bound_keeps_insertion_order_and_descriptions() {
        let mut g = SystemGraph::default();
        g.bind(item(Layer::Data, "b"), item(Layer::App, "y"), "second added first");
        g.bind(item(Layer::Data, "a"), item(Layer::App, "x"), "first added second");
        let got: Vec<(&str, &str)> = g.bindings().map(|(b, _)| (b.from.key.as_str(), b.description.as_str())).collect();
        assert_eq!(got, vec![("b", "second added first"), ("a", "first added second")]);
    }

    #[test]
    fn bindings_empty_graph_returns_nothing() {
        assert_eq!(SystemGraph::default().bindings().count(), 0);
    }

    #[test]
    fn impact_unknown_item_returns_empty() {
        let g = claims();
        assert!(g.impact(&item(Layer::App, "Nowhere")).is_empty());
    }

    #[test]
    fn impact_chain_returns_breadth_first_order() {
        let mut g = SystemGraph::default();
        let (a, b, c, d) = (
            item(Layer::Data, "a"),
            item(Layer::Data, "b"),
            item(Layer::App, "c"),
            item(Layer::Architecture, "d"),
        );
        g.bind(a.clone(), b.clone(), "");
        g.bind(b.clone(), d.clone(), "");
        g.bind(a.clone(), c.clone(), "");
        assert_eq!(g.impact(&a), vec![b, c, d]);
    }

    #[test]
    fn impact_diamond_lists_shared_target_once() {
        let mut g = SystemGraph::default();
        let (a, b, c, d) = (
            item(Layer::Data, "a"),
            item(Layer::App, "b"),
            item(Layer::App, "c"),
            item(Layer::Architecture, "d"),
        );
        g.bind(a.clone(), b.clone(), "");
        g.bind(a.clone(), c.clone(), "");
        g.bind(b, d.clone(), "");
        g.bind(c, d.clone(), "");
        assert_eq!(g.impact(&a).iter().filter(|i| **i == d).count(), 1);
    }

    #[test]
    fn impact_cycle_terminates_with_each_item_once() {
        let mut g = SystemGraph::default();
        let (a, b, c) = (item(Layer::Data, "a"), item(Layer::App, "b"), item(Layer::Architecture, "c"));
        g.bind(a.clone(), b.clone(), "");
        g.bind(b.clone(), c.clone(), "");
        g.bind(c.clone(), b.clone(), "");
        assert_eq!(g.impact(&a), vec![b, c]);
    }

    #[test]
    fn impact_does_not_follow_bindings_backwards() {
        let g = claims();
        assert!(g.impact(&item(Layer::App, "StatusBadge")).is_empty());
    }

    #[test]
    fn deploy_order_no_changes_returns_empty_plan() {
        assert!(claims().deploy_order(&[]).is_empty());
    }

    #[test]
    fn deploy_order_unbound_change_returns_only_its_layer() {
        let g = SystemGraph::default();
        let x = item(Layer::App, "Standalone");
        assert_eq!(g.deploy_order(std::slice::from_ref(&x)), vec![(Layer::App, vec![x])]);
    }

    #[test]
    fn deploy_order_claims_change_groups_items_sorted_within_layers() {
        let plan = claims().deploy_order(&[item(Layer::Data, "Claim.status")]);
        assert_eq!(
            plan,
            vec![
                (Layer::Architecture, vec![item(Layer::Architecture, "sql-claims-stg")]),
                (
                    Layer::Data,
                    vec![item(Layer::Data, "Claim.status"), item(Layer::Data, "migration 0007")]
                ),
                (Layer::App, vec![item(Layer::App, "StatusBadge")]),
            ]
        );
    }

    #[test]
    fn deploy_order_overlapping_changes_lists_each_item_once() {
        let g = claims();
        let plan = g.deploy_order(&[item(Layer::Data, "Claim.status"), item(Layer::Data, "migration 0007")]);
        let total: usize = plan.iter().map(|(_, v)| v.len()).sum();
        assert_eq!(total, 4);
    }
}
