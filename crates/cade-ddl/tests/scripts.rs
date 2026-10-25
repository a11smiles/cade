//! Full schema and migration scripts compared with checked-in golden files.
//!
//! Oracle has no golden files yet: its column definitions put `DEFAULT` after
//! `NOT NULL`/`PRIMARY KEY`, which Oracle rejects, so a golden would lock in
//! invalid SQL. Oracle's type mapping and quoting are unit-tested in `src/lib.rs`.

mod common;

use cade_ddl::{Dialect, check_pii, migration, schema_sql};
use common::{v1, v2};

fn golden(name: &str) -> String {
    let path = format!("{}/tests/golden/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

fn with_golden() -> impl Iterator<Item = (Dialect, &'static str)> {
    common::DIALECTS.into_iter().filter(|(d, _)| *d != Dialect::Oracle)
}

#[test]
fn schema_sql_every_type_matches_golden_per_dialect() {
    for (d, n) in with_golden() {
        assert_eq!(schema_sql(d, &v1()).unwrap(), golden(&format!("schema.{n}.sql")), "{n}");
    }
}

#[test]
fn migration_up_matches_golden_per_dialect() {
    for (d, n) in with_golden() {
        assert_eq!(
            migration(d, &v1(), &v2()).unwrap().up,
            golden(&format!("migration.{n}.up.sql")),
            "{n}"
        );
    }
}

#[test]
fn migration_down_matches_golden_per_dialect() {
    for (d, n) in with_golden() {
        assert_eq!(
            migration(d, &v1(), &v2()).unwrap().down,
            golden(&format!("migration.{n}.down.sql")),
            "{n}"
        );
    }
}

#[test]
fn migration_down_undoes_up_in_reverse_order() {
    for (d, n) in common::DIALECTS {
        let m = migration(d, &v1(), &v2()).unwrap();
        let up: Vec<&str> = m.up.lines().filter(|l| l.starts_with("CREATE") || l.starts_with("ALTER")).collect();
        let down: Vec<&str> = m.down.lines().collect();
        assert_eq!(up.len(), down.len(), "{n}: every up step has a down step");
        // The last thing created is the first thing dropped.
        assert!(up.last().unwrap().starts_with("CREATE TABLE"), "{n}");
        assert!(down[0].starts_with("DROP TABLE"), "{n}");
        assert!(down[0].contains("shipments"), "{n}");
    }
}

#[test]
fn migration_rewrites_table_only_where_now_default_is_not_constant() {
    let got: Vec<bool> = common::DIALECTS
        .iter()
        .map(|(d, _)| migration(*d, &v1(), &v2()).unwrap().rewrites_table)
        .collect();
    assert_eq!(got, [false, true, true, true]);
}

#[test]
fn check_pii_classified_model_passes() {
    assert!(check_pii(&v2()).is_empty());
}

#[test]
fn model_serde_field_types_and_defaults_use_kebab_case_tags() {
    let json = serde_json::to_value(v1()).unwrap();
    let fields = &json["entities"][1]["fields"];
    assert_eq!(fields[2]["ty"], serde_json::json!({ "t": "decimal", "precision": 12, "scale": 2 }));
    assert_eq!(fields[3]["ty"], serde_json::json!({ "t": "enum", "name": "order_state" }));
    assert_eq!(json["entities"][0]["fields"][1]["ty"], serde_json::json!({ "t": "ci-text" }));
    assert_eq!(json["entities"][0]["fields"][1]["classification"], "pii");
    assert_eq!(
        json["entities"][0]["fields"][0]["default"],
        serde_json::json!({ "t": "generated-uuid" })
    );
    assert_eq!(serde_json::from_value::<cade_ddl::Model>(json).unwrap(), v1());
}
