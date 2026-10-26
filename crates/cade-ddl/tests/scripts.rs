//! Full schema and migration scripts compared with checked-in golden files.

mod common;

use cade_ddl::{DefaultValue, Dialect, Entity, EnumType, Field, FieldType, Model, check_pii, create_table, migration, schema_sql};
use common::{v1, v2};

fn golden(name: &str) -> String {
    let path = format!("{}/tests/golden/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

fn with_golden() -> impl Iterator<Item = (Dialect, &'static str)> {
    common::DIALECTS.into_iter()
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
fn migration_sqlserver_long_defaults_preserves_distinct_constraint_identity() {
    // Include the reported collision and names just below, at and above 128 characters.
    for len in [125, 122, 123, 124, 126] {
        let table = "t".repeat(len);
        let from = Model {
            enums: vec![],
            entities: vec![Entity {
                name: "Item".into(),
                schema: "s".into(),
                table: table.clone(),
                fields: vec![Field::new("kept", FieldType::Int).default(DefaultValue::Literal { sql: "7".into() })],
            }],
        };
        let mut to = from.clone();
        to.entities[0].fields.extend([
            Field::new("a", FieldType::Int).default(DefaultValue::Literal { sql: "0".into() }),
            Field::new("b", FieldType::Int).default(DefaultValue::Literal { sql: "1".into() }),
        ]);
        let sql = create_table(Dialect::SqlServer, &to.entities[0], &to).unwrap();
        let names: Vec<&str> = sql.split("CONSTRAINT [").skip(1).map(|s| s.split(']').next().unwrap()).collect();
        assert_eq!(names.len(), 3, "{sql}");
        assert_ne!(names[1], names[2], "table length {len}: {sql}");
        assert_ne!(names[0], names[1], "{sql}");
        assert_ne!(names[0], names[2], "{sql}");
        assert!(names.iter().all(|name| !name.is_empty() && name.chars().count() <= 128), "{sql}");
        assert!(
            sql.contains(&format!("[a] int CONSTRAINT [{}] DEFAULT 0 NOT NULL", names[1])),
            "{sql}"
        );
        assert!(
            sql.contains(&format!("[b] int CONSTRAINT [{}] DEFAULT 1 NOT NULL", names[2])),
            "{sql}"
        );
        for (index, column) in [(1, "a"), (2, "b")] {
            if len + 5 <= 128 {
                assert_eq!(names[index], format!("df_{table}_{column}"));
            }
        }
        let m = migration(Dialect::SqlServer, &from, &to).unwrap();
        assert_eq!(
            m.up,
            format!(
                "ALTER TABLE [s].[{table}] ADD [a] int CONSTRAINT [{}] DEFAULT 0 NOT NULL;\n\
                 ALTER TABLE [s].[{table}] ADD [b] int CONSTRAINT [{}] DEFAULT 1 NOT NULL;\n",
                names[1], names[2]
            )
        );
        assert_eq!(
            m.down,
            format!(
                "ALTER TABLE [s].[{table}] DROP CONSTRAINT [{}], COLUMN [b];\n\
                 ALTER TABLE [s].[{table}] DROP CONSTRAINT [{}], COLUMN [a];\n",
                names[2], names[1]
            )
        );
        assert_eq!(create_table(Dialect::SqlServer, &to.entities[0], &to).unwrap(), sql);
        assert_eq!(migration(Dialect::SqlServer, &from, &to).unwrap(), m);
    }
}

#[test]
fn migration_sqlserver_long_enum_checks_preserves_distinct_constraint_identity() {
    for len in [125, 122, 123, 124, 126] {
        let table = "t".repeat(len);
        let from = Model {
            enums: vec![
                EnumType {
                    schema: "s".into(),
                    name: "first".into(),
                    values: vec!["open".into(), "can't".into()],
                },
                EnumType {
                    schema: "s".into(),
                    name: "second".into(),
                    values: vec!["paid".into(), "closed".into()],
                },
            ],
            entities: vec![Entity {
                name: "Item".into(),
                schema: "s".into(),
                table: table.clone(),
                fields: vec![Field::new("kept", FieldType::Int)],
            }],
        };
        let mut to = from.clone();
        to.entities[0].fields.extend([
            Field::new("a", FieldType::Enum { name: "first".into() }),
            Field::new("b", FieldType::Enum { name: "second".into() }),
        ]);
        let sql = create_table(Dialect::SqlServer, &to.entities[0], &to).unwrap();
        let names: Vec<&str> = sql.split("CONSTRAINT [").skip(1).map(|s| s.split(']').next().unwrap()).collect();
        assert_eq!(names.len(), 2, "{sql}");
        assert_ne!(names[0], names[1], "table length {len}: {sql}");
        assert!(names.iter().all(|name| !name.is_empty() && name.chars().count() <= 128), "{sql}");
        let a = format!(
            "[a] nvarchar(32) NOT NULL CONSTRAINT [{}] CHECK ([a] IN ('open', 'can''t'))",
            names[0]
        );
        let b = format!(
            "[b] nvarchar(32) NOT NULL CONSTRAINT [{}] CHECK ([b] IN ('paid', 'closed'))",
            names[1]
        );
        assert!(sql.contains(&a), "{sql}");
        assert!(sql.contains(&b), "{sql}");
        for (index, column) in [(0, "a"), (1, "b")] {
            if len + 5 <= 128 {
                assert_eq!(names[index], format!("ck_{table}_{column}"));
            }
        }
        let m = migration(Dialect::SqlServer, &from, &to).unwrap();
        assert_eq!(
            m.up,
            format!("ALTER TABLE [s].[{table}] ADD {a};\nALTER TABLE [s].[{table}] ADD {b};\n")
        );
        assert_eq!(
            m.down,
            format!(
                "ALTER TABLE [s].[{table}] DROP CONSTRAINT [{}], COLUMN [b];\n\
                 ALTER TABLE [s].[{table}] DROP CONSTRAINT [{}], COLUMN [a];\n",
                names[1], names[0]
            )
        );
        assert_eq!(create_table(Dialect::SqlServer, &to.entities[0], &to).unwrap(), sql);
        assert_eq!(migration(Dialect::SqlServer, &from, &to).unwrap(), m);
    }
}

#[test]
fn migration_mysql_long_foreign_keys_preserves_distinct_constraint_identity() {
    // parent_a/parent_b produce 63-, 64- and 65-character ordinary names at 51/52/53.
    for len in [61, 51, 52, 53, 60, 62] {
        let table = "t".repeat(len);
        let from = Model {
            enums: vec![],
            entities: vec![Entity {
                name: "Item".into(),
                schema: "s".into(),
                table: table.clone(),
                fields: vec![Field::new("kept", FieldType::Uuid).references("s", "keepers", "id")],
            }],
        };
        let mut to = from.clone();
        to.entities[0].fields.extend([
            Field::new("parent_a", FieldType::Uuid).references("s", "parents_a", "id"),
            Field::new("parent_b", FieldType::Uuid).references("s", "parents_b", "id"),
        ]);
        let sql = create_table(Dialect::MySql, &to.entities[0], &to).unwrap();
        let names: Vec<&str> = sql.split("CONSTRAINT `").skip(1).map(|s| s.split('`').next().unwrap()).collect();
        assert_eq!(names.len(), 3, "{sql}");
        assert_ne!(names[1], names[2], "table length {len}: {sql}");
        assert_ne!(names[0], names[1], "{sql}");
        assert_ne!(names[0], names[2], "{sql}");
        assert!(names.iter().all(|name| !name.is_empty() && name.chars().count() <= 64), "{sql}");
        let mut up = String::new();
        let mut down = Vec::new();
        for (index, column, target) in [(1, "parent_a", "parents_a"), (2, "parent_b", "parents_b")] {
            let name = names[index];
            if len + 12 <= 64 {
                assert_eq!(name, format!("fk_{table}_{column}"));
            }
            let fk = format!("CONSTRAINT `{name}` FOREIGN KEY (`{column}`) REFERENCES `s`.`{target}`(`id`)");
            assert!(sql.contains(&format!("  {fk}")), "{sql}");
            up.push_str(&format!(
                "ALTER TABLE `s`.`{table}` ADD COLUMN `{column}` char(36) NOT NULL, ADD {fk};\n"
            ));
            down.push(format!(
                "ALTER TABLE `s`.`{table}` DROP FOREIGN KEY `{name}`, DROP COLUMN `{column}`;"
            ));
        }
        down.reverse();
        let m = migration(Dialect::MySql, &from, &to).unwrap();
        assert_eq!(m.up, up);
        assert_eq!(m.down, down.join("\n") + "\n");
        assert_eq!(create_table(Dialect::MySql, &to.entities[0], &to).unwrap(), sql);
        assert_eq!(migration(Dialect::MySql, &from, &to).unwrap(), m);
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
