//! Shared synthetic models: `v1` uses every field type and default kind;
//! `v2` adds an enum, a table and columns, so the migration covers each change.

use cade_ddl::*;

pub const DIALECTS: [(Dialect, &str); 4] = [
    (Dialect::PostgreSql, "postgresql"),
    (Dialect::SqlServer, "sqlserver"),
    (Dialect::MySql, "mysql"),
    (Dialect::Oracle, "oracle"),
];

pub fn v1() -> Model {
    Model {
        enums: vec![EnumType {
            schema: "shop".into(),
            name: "order_state".into(),
            values: vec!["open".into(), "paid".into()],
        }],
        entities: vec![
            Entity {
                name: "Customer".into(),
                schema: "shop".into(),
                table: "customers".into(),
                fields: vec![
                    Field::new("id", FieldType::Uuid).pk(),
                    Field::new("email", FieldType::CiText).classify(Classification::Pii),
                    Field::new("bio", FieldType::Text).nullable(),
                    Field::new("visits", FieldType::Int).default(DefaultValue::Literal { sql: "0".into() }),
                    Field::new("lifetime_cents", FieldType::BigInt),
                    Field::new("active", FieldType::Bool),
                    Field::new("joined_on", FieldType::Date),
                    Field::new("created_at", FieldType::Timestamptz).default(DefaultValue::Now),
                    Field::new("avatar", FieldType::Bytes).nullable(),
                ],
            },
            Entity {
                name: "Order".into(),
                schema: "shop".into(),
                table: "orders".into(),
                fields: vec![
                    Field::new("id", FieldType::Uuid).pk(),
                    Field::new("customer_id", FieldType::Uuid).references("shop", "customers", "id"),
                    Field::new("total", FieldType::Decimal { precision: 12, scale: 2 }),
                    Field::new(
                        "state",
                        FieldType::Enum {
                            name: "order_state".into(),
                        },
                    ),
                ],
            },
        ],
    }
}

pub fn v2() -> Model {
    let mut m = v1();
    m.enums.push(EnumType {
        schema: "shop".into(),
        name: "ship_method".into(),
        values: vec!["ground".into(), "air".into()],
    });
    m.entities[1].fields.push(Field::new("note", FieldType::Text).nullable());
    m.entities[1]
        .fields
        .push(Field::new("placed_at", FieldType::Timestamptz).default(DefaultValue::Now));
    m.entities.push(Entity {
        name: "Shipment".into(),
        schema: "shop".into(),
        table: "shipments".into(),
        fields: vec![
            Field::new("id", FieldType::Uuid).pk(),
            Field::new("order_id", FieldType::Uuid).references("shop", "orders", "id"),
            Field::new(
                "method",
                FieldType::Enum {
                    name: "ship_method".into(),
                },
            ),
        ],
    });
    m
}
