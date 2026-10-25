//! The System view end to end: bindings table, impact of a change and the deploy plan it implies.

use cade_system::{Item, Layer, Status, SystemGraph};

fn i(layer: Layer, key: &str) -> Item {
    Item::new(layer, key)
}

#[test]
fn system_graph_schema_change_flows_to_bindings_impact_and_deploy_plan() {
    let mut g = SystemGraph::default();
    for it in [
        i(Layer::Data, "Claim.status"),
        i(Layer::Data, "migration 0008"),
        i(Layer::App, "StatusBadge"),
        i(Layer::App, "ClaimList"),
        i(Layer::Architecture, "sql-claims"),
        i(Layer::Architecture, "app-claims"),
    ] {
        g.add_item(it);
    }
    g.bind(
        i(Layer::Data, "Claim.status"),
        i(Layer::App, "StatusBadge"),
        "Enum renders as a badge",
    );
    g.bind(
        i(Layer::App, "StatusBadge"),
        i(Layer::App, "ClaimList"),
        "Badge is used in the list",
    );
    g.bind(i(Layer::Data, "Claim.status"), i(Layer::Data, "migration 0008"), "Schema change");
    g.bind(
        i(Layer::Data, "migration 0008"),
        i(Layer::Architecture, "sql-claims"),
        "Runs against the database",
    );
    g.bind(
        i(Layer::App, "ClaimList"),
        i(Layer::Architecture, "app-claims"),
        "Deployed to the web app",
    );
    g.bind(
        i(Layer::App, "ClaimList"),
        i(Layer::Architecture, "setting:API_URL"),
        "App reads this setting",
    );
    g.mark_pending(i(Layer::Data, "migration 0008"));

    let table: Vec<(String, Status)> = g.bindings().map(|(b, s)| (b.description.clone(), s)).collect();
    assert_eq!(
        table,
        vec![
            ("Enum renders as a badge".to_string(), Status::InSync),
            ("Badge is used in the list".to_string(), Status::InSync),
            ("Schema change".to_string(), Status::Pending),
            ("Runs against the database".to_string(), Status::Pending),
            ("Deployed to the web app".to_string(), Status::InSync),
            ("App reads this setting".to_string(), Status::Missing),
        ]
    );

    let changed = i(Layer::Data, "Claim.status");
    assert_eq!(
        g.impact(&changed),
        vec![
            i(Layer::App, "StatusBadge"),
            i(Layer::Data, "migration 0008"),
            i(Layer::App, "ClaimList"),
            i(Layer::Architecture, "sql-claims"),
            i(Layer::Architecture, "app-claims"),
            i(Layer::Architecture, "setting:API_URL"),
        ]
    );

    assert_eq!(
        g.deploy_order(&[changed]),
        vec![
            (
                Layer::Architecture,
                vec![
                    i(Layer::Architecture, "app-claims"),
                    i(Layer::Architecture, "setting:API_URL"),
                    i(Layer::Architecture, "sql-claims"),
                ]
            ),
            (Layer::Data, vec![i(Layer::Data, "Claim.status"), i(Layer::Data, "migration 0008")]),
            (Layer::App, vec![i(Layer::App, "ClaimList"), i(Layer::App, "StatusBadge")]),
        ]
    );
}
