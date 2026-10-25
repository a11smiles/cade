//! Data model → DDL and reversible migrations.
//!
//! The Data model workspace edits a [`Model`]. This crate renders it as
//! `CREATE` statements for four dialects and diffs two models into a
//! [`Migration`] with matching up and down scripts.
//!
//! Backlog: DATA epic.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Dialect {
    PostgreSql,
    SqlServer,
    MySql,
    Oracle,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", rename_all = "kebab-case")]
pub enum FieldType {
    Uuid,
    Text,
    /// Case-insensitive text (PostgreSQL `citext`; others fall back to text with a CI collation note).
    CiText,
    Int,
    BigInt,
    Bool,
    Date,
    Timestamptz,
    Decimal {
        precision: u8,
        scale: u8,
    },
    Bytes,
    Enum {
        name: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Classification {
    Public,
    Internal,
    Confidential,
    Pii,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub schema: String,
    pub table: String,
    pub column: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub ty: FieldType,
    pub nullable: bool,
    pub primary_key: bool,
    /// Raw SQL default, written per dialect by [`default_sql`].
    pub default: Option<DefaultValue>,
    pub references: Option<Reference>,
    pub classification: Option<Classification>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", rename_all = "kebab-case")]
pub enum DefaultValue {
    GeneratedUuid,
    Now,
    Literal { sql: String },
}

impl Field {
    pub fn new(name: &str, ty: FieldType) -> Self {
        Field {
            name: name.into(),
            ty,
            nullable: false,
            primary_key: false,
            default: None,
            references: None,
            classification: None,
        }
    }
    pub fn pk(mut self) -> Self {
        self.primary_key = true;
        self.default = Some(DefaultValue::GeneratedUuid);
        self
    }
    pub fn nullable(mut self) -> Self {
        self.nullable = true;
        self
    }
    pub fn default(mut self, d: DefaultValue) -> Self {
        self.default = Some(d);
        self
    }
    pub fn references(mut self, schema: &str, table: &str, column: &str) -> Self {
        self.references = Some(Reference {
            schema: schema.into(),
            table: table.into(),
            column: column.into(),
        });
        self
    }
    pub fn classify(mut self, c: Classification) -> Self {
        self.classification = Some(c);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub schema: String,
    pub table: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumType {
    pub schema: String,
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Model {
    pub enums: Vec<EnumType>,
    pub entities: Vec<Entity>,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum DdlError {
    #[error("{entity}.{field} holds personal data but has no classification")]
    UnclassifiedPii { entity: String, field: String },
    #[error("unknown enum {0}")]
    UnknownEnum(String),
}

fn q(d: Dialect, ident: &str) -> String {
    match d {
        Dialect::PostgreSql | Dialect::Oracle => ident.to_string(),
        Dialect::SqlServer => format!("[{ident}]"),
        Dialect::MySql => format!("`{ident}`"),
    }
}

fn qualified(d: Dialect, schema: &str, name: &str) -> String {
    format!("{}.{}", q(d, schema), q(d, name))
}

/// SQL type for a field in a dialect.
pub fn type_sql(d: Dialect, ty: &FieldType, model: &Model) -> Result<String, DdlError> {
    use Dialect::*;
    use FieldType as F;
    Ok(match (d, ty) {
        (PostgreSql, F::Uuid) => "uuid".into(),
        (SqlServer, F::Uuid) => "uniqueidentifier".into(),
        (MySql, F::Uuid) => "char(36)".into(),
        (Oracle, F::Uuid) => "raw(16)".into(),
        (PostgreSql, F::Text) => "text".into(),
        (SqlServer, F::Text) => "nvarchar(max)".into(),
        (MySql, F::Text) => "text".into(),
        (Oracle, F::Text) => "clob".into(),
        (PostgreSql, F::CiText) => "citext".into(),
        (SqlServer, F::CiText) => "nvarchar(320) COLLATE Latin1_General_CI_AS".into(),
        (MySql, F::CiText) => "varchar(320)".into(),
        (Oracle, F::CiText) => "varchar2(320)".into(),
        (Oracle, F::Int) => "number(10)".into(),
        (_, F::Int) => "int".into(),
        (Oracle, F::BigInt) => "number(19)".into(),
        (_, F::BigInt) => "bigint".into(),
        (PostgreSql, F::Bool) => "boolean".into(),
        (SqlServer, F::Bool) => "bit".into(),
        (MySql, F::Bool) => "tinyint(1)".into(),
        (Oracle, F::Bool) => "number(1)".into(),
        (_, F::Date) => "date".into(),
        (PostgreSql, F::Timestamptz) => "timestamptz".into(),
        (SqlServer, F::Timestamptz) => "datetimeoffset".into(),
        (MySql, F::Timestamptz) => "timestamp".into(),
        (Oracle, F::Timestamptz) => "timestamp with time zone".into(),
        (Oracle, F::Decimal { precision, scale }) => format!("number({precision},{scale})"),
        (_, F::Decimal { precision, scale }) => format!("decimal({precision},{scale})"),
        (PostgreSql, F::Bytes) => "bytea".into(),
        (SqlServer, F::Bytes) => "varbinary(max)".into(),
        (MySql, F::Bytes) => "blob".into(),
        (Oracle, F::Bytes) => "blob".into(),
        (_, F::Enum { name }) => {
            let e = model
                .enums
                .iter()
                .find(|e| &e.name == name)
                .ok_or_else(|| DdlError::UnknownEnum(name.clone()))?;
            match d {
                PostgreSql => qualified(d, &e.schema, &e.name),
                MySql => format!("enum({})", e.values.iter().map(|v| format!("'{v}'")).collect::<Vec<_>>().join(", ")),
                SqlServer => "nvarchar(32)".into(),
                Oracle => "varchar2(32)".into(),
            }
        }
    })
}

fn default_sql(d: Dialect, def: &DefaultValue) -> String {
    match (d, def) {
        (Dialect::PostgreSql, DefaultValue::GeneratedUuid) => "gen_random_uuid()".into(),
        (Dialect::SqlServer, DefaultValue::GeneratedUuid) => "newsequentialid()".into(),
        (Dialect::MySql, DefaultValue::GeneratedUuid) => "(uuid())".into(),
        (Dialect::Oracle, DefaultValue::GeneratedUuid) => "sys_guid()".into(),
        (Dialect::PostgreSql, DefaultValue::Now) => "now()".into(),
        (Dialect::SqlServer, DefaultValue::Now) => "sysdatetimeoffset()".into(),
        (Dialect::MySql, DefaultValue::Now) => "CURRENT_TIMESTAMP".into(),
        (Dialect::Oracle, DefaultValue::Now) => "systimestamp".into(),
        (_, DefaultValue::Literal { sql }) => sql.clone(),
    }
}

fn column_sql(d: Dialect, f: &Field, model: &Model) -> Result<String, DdlError> {
    let mut s = format!("{} {}", q(d, &f.name), type_sql(d, &f.ty, model)?);
    if f.primary_key {
        s.push_str(" PRIMARY KEY");
    } else if !f.nullable {
        s.push_str(" NOT NULL");
    }
    if let Some(def) = &f.default {
        s.push_str(&format!(" DEFAULT {}", default_sql(d, def)));
    }
    if let Some(r) = &f.references {
        s.push_str(&format!(" REFERENCES {}({})", qualified(d, &r.schema, &r.table), q(d, &r.column)));
    }
    Ok(s)
}

pub fn create_enum(d: Dialect, e: &EnumType) -> Option<String> {
    match d {
        Dialect::PostgreSql => Some(format!(
            "CREATE TYPE {} AS ENUM ({});",
            qualified(d, &e.schema, &e.name),
            e.values.iter().map(|v| format!("'{v}'")).collect::<Vec<_>>().join(", ")
        )),
        _ => None, // inline enum (MySQL) or check-constrained string (SQL Server, Oracle)
    }
}

pub fn create_table(d: Dialect, e: &Entity, model: &Model) -> Result<String, DdlError> {
    let cols: Result<Vec<_>, _> = e.fields.iter().map(|f| column_sql(d, f, model).map(|c| format!("  {c}"))).collect();
    Ok(format!(
        "CREATE TABLE {} (\n{}\n);",
        qualified(d, &e.schema, &e.table),
        cols?.join(",\n")
    ))
}

/// Full schema script, enums first.
pub fn schema_sql(d: Dialect, model: &Model) -> Result<String, DdlError> {
    let mut out = vec![];
    for e in &model.enums {
        if let Some(s) = create_enum(d, e) {
            out.push(s);
        }
    }
    for e in &model.entities {
        out.push(create_table(d, e, model)?);
    }
    Ok(out.join("\n\n") + "\n")
}

/// Policy check: fields that look like personal data must be classified.
pub fn check_pii(model: &Model) -> Vec<DdlError> {
    const HINTS: [&str; 8] = ["name", "email", "phone", "address", "ssn", "birth", "dob", "passport"];
    let mut errs = vec![];
    for e in &model.entities {
        for f in &e.fields {
            let lower = f.name.to_lowercase();
            if f.classification.is_none() && HINTS.iter().any(|h| lower.contains(h)) {
                errs.push(DdlError::UnclassifiedPii {
                    entity: e.name.clone(),
                    field: f.name.clone(),
                });
            }
        }
    }
    errs
}

#[derive(Debug, Clone, PartialEq)]
pub enum Change {
    AddEnum(EnumType),
    AddEntity(Entity),
    AddField { entity: Entity, field: Field },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Migration {
    pub up: String,
    pub down: String,
    /// True when a step would rewrite a large table (e.g. a non-constant default).
    pub rewrites_table: bool,
}

/// Diff `from` → `to`. Covers additions; drops and renames are DATA backlog items.
pub fn diff(from: &Model, to: &Model) -> Vec<Change> {
    let mut changes = vec![];
    for e in &to.enums {
        if !from.enums.iter().any(|x| x.schema == e.schema && x.name == e.name) {
            changes.push(Change::AddEnum(e.clone()));
        }
    }
    for e in &to.entities {
        match from.entities.iter().find(|x| x.schema == e.schema && x.table == e.table) {
            None => changes.push(Change::AddEntity(e.clone())),
            Some(old) => {
                for f in &e.fields {
                    if !old.fields.iter().any(|x| x.name == f.name) {
                        changes.push(Change::AddField {
                            entity: e.clone(),
                            field: f.clone(),
                        });
                    }
                }
            }
        }
    }
    changes
}

pub fn migration(d: Dialect, from: &Model, to: &Model) -> Result<Migration, DdlError> {
    let mut up = vec![];
    let mut down = vec![];
    let mut rewrites = false;
    for c in diff(from, to) {
        match c {
            Change::AddEnum(e) => {
                if let Some(s) = create_enum(d, &e) {
                    up.push(s);
                    down.push(format!("DROP TYPE {};", qualified(d, &e.schema, &e.name)));
                }
            }
            Change::AddEntity(e) => {
                up.push(create_table(d, &e, to)?);
                down.push(format!("DROP TABLE {};", qualified(d, &e.schema, &e.table)));
            }
            Change::AddField { entity, field } => {
                if !field.nullable && field.default.is_none() {
                    rewrites = true; // needs a backfill
                }
                if matches!(field.default, Some(DefaultValue::GeneratedUuid) | Some(DefaultValue::Now)) && d != Dialect::PostgreSql {
                    rewrites = true;
                }
                let kw = if d == Dialect::Oracle { "ADD" } else { "ADD COLUMN" };
                let kw = if d == Dialect::SqlServer { "ADD" } else { kw };
                up.push(format!(
                    "ALTER TABLE {} {kw} {};",
                    qualified(d, &entity.schema, &entity.table),
                    column_sql(d, &field, to)?
                ));
                down.push(format!(
                    "ALTER TABLE {} DROP COLUMN {};",
                    qualified(d, &entity.schema, &entity.table),
                    q(d, &field.name)
                ));
            }
        }
    }
    down.reverse();
    Ok(Migration {
        up: up.join("\n") + "\n",
        down: down.join("\n") + "\n",
        rewrites_table: rewrites,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v6() -> Model {
        Model {
            enums: vec![EnumType {
                schema: "intake".into(),
                name: "claim_type".into(),
                values: vec!["auto".into(), "property".into()],
            }],
            entities: vec![Entity {
                name: "Claim".into(),
                schema: "intake".into(),
                table: "claims".into(),
                fields: vec![
                    Field::new("id", FieldType::Uuid).pk(),
                    Field::new("incident_date", FieldType::Date),
                    Field::new("incident_type", FieldType::Enum { name: "claim_type".into() }),
                ],
            }],
        }
    }

    fn v7() -> Model {
        let mut m = v6();
        m.enums.push(EnumType {
            schema: "intake".into(),
            name: "claim_status".into(),
            values: ["draft", "submitted", "in_review", "approved", "denied", "closed"]
                .map(String::from)
                .to_vec(),
        });
        m.entities[0].fields.push(
            Field::new(
                "status",
                FieldType::Enum {
                    name: "claim_status".into(),
                },
            )
            .default(DefaultValue::Literal { sql: "'draft'".into() }),
        );
        m.entities.push(Entity {
            name: "ClaimStatusHistory".into(),
            schema: "intake".into(),
            table: "claim_status_history".into(),
            fields: vec![
                Field::new("id", FieldType::Uuid).pk(),
                Field::new("claim_id", FieldType::Uuid).references("intake", "claims", "id"),
                Field::new(
                    "to_status",
                    FieldType::Enum {
                        name: "claim_status".into(),
                    },
                ),
                Field::new("changed_at", FieldType::Timestamptz).default(DefaultValue::Now),
            ],
        });
        m
    }

    #[test]
    fn postgres_schema() {
        let s = schema_sql(Dialect::PostgreSql, &v6()).unwrap();
        assert!(s.contains("CREATE TYPE intake.claim_type AS ENUM ('auto', 'property');"));
        assert!(s.contains("id uuid PRIMARY KEY DEFAULT gen_random_uuid()"));
    }

    #[test]
    fn migration_0007_matches_the_screen() {
        let m = migration(Dialect::PostgreSql, &v6(), &v7()).unwrap();
        assert!(
            m.up.contains("CREATE TYPE intake.claim_status AS ENUM ('draft', 'submitted', 'in_review', 'approved', 'denied', 'closed');")
        );
        assert!(m.up.contains("ALTER TABLE intake.claims ADD COLUMN status intake.claim_status NOT NULL DEFAULT 'draft';"));
        assert!(m.up.contains("REFERENCES intake.claims(id)"));
        assert!(!m.rewrites_table, "constant default must not rewrite on PostgreSQL 11+");
        let first_down = m.down.lines().next().unwrap();
        assert!(first_down.starts_with("DROP TABLE intake.claim_status_history"));
        assert!(m.down.trim_end().ends_with("DROP TYPE intake.claim_status;"));
    }

    #[test]
    fn other_dialects_quote_and_map_types() {
        let s = schema_sql(Dialect::SqlServer, &v7()).unwrap();
        assert!(s.contains("[id] uniqueidentifier PRIMARY KEY DEFAULT newsequentialid()"));
        let s = schema_sql(Dialect::MySql, &v7()).unwrap();
        assert!(s.contains("`incident_type` enum('auto', 'property')"));
        let s = schema_sql(Dialect::Oracle, &v7()).unwrap();
        assert!(s.contains("changed_at timestamp with time zone NOT NULL DEFAULT systimestamp"));
    }

    #[test]
    fn flags_unclassified_pii() {
        let mut m = v6();
        m.entities[0].fields.push(Field::new("email", FieldType::CiText));
        assert_eq!(check_pii(&m).len(), 1);
        m.entities[0].fields.last_mut().unwrap().classification = Some(Classification::Pii);
        assert!(check_pii(&m).is_empty());
    }

    const ALL: [Dialect; 4] = [Dialect::PostgreSql, Dialect::SqlServer, Dialect::MySql, Dialect::Oracle];

    fn entity(table: &str, fields: Vec<Field>) -> Entity {
        Entity {
            name: table.into(),
            schema: "s".into(),
            table: table.into(),
            fields,
        }
    }

    #[test]
    fn field_new_defaults_to_required_plain_column() {
        let f = Field::new("x", FieldType::Int);
        assert_eq!(
            f,
            Field {
                name: "x".into(),
                ty: FieldType::Int,
                nullable: false,
                primary_key: false,
                default: None,
                references: None,
                classification: None,
            }
        );
    }

    #[test]
    fn field_pk_sets_primary_key_and_generated_uuid_default() {
        let f = Field::new("id", FieldType::Uuid).pk();
        assert!(f.primary_key);
        assert_eq!(f.default, Some(DefaultValue::GeneratedUuid));
    }

    #[test]
    fn field_builders_set_nullable_reference_and_classification() {
        let f = Field::new("owner_id", FieldType::Uuid)
            .nullable()
            .references("app", "users", "id")
            .classify(Classification::Internal);
        assert!(f.nullable);
        assert_eq!(
            f.references,
            Some(Reference {
                schema: "app".into(),
                table: "users".into(),
                column: "id".into()
            })
        );
        assert_eq!(f.classification, Some(Classification::Internal));
    }

    #[test]
    fn q_each_dialect_uses_its_identifier_quoting() {
        let got: Vec<String> = ALL.iter().map(|d| q(*d, "order")).collect();
        assert_eq!(got, ["order", "[order]", "`order`", "order"]);
    }

    #[test]
    fn qualified_joins_schema_and_name_quoted_separately() {
        assert_eq!(qualified(Dialect::SqlServer, "dbo", "users"), "[dbo].[users]");
        assert_eq!(qualified(Dialect::MySql, "app", "users"), "`app`.`users`");
        assert_eq!(qualified(Dialect::PostgreSql, "app", "users"), "app.users");
    }

    #[test]
    fn type_sql_every_type_maps_per_dialect() {
        let m = v6();
        let types = [
            FieldType::Uuid,
            FieldType::Text,
            FieldType::CiText,
            FieldType::Int,
            FieldType::BigInt,
            FieldType::Bool,
            FieldType::Date,
            FieldType::Timestamptz,
            FieldType::Decimal { precision: 10, scale: 4 },
            FieldType::Bytes,
            FieldType::Enum { name: "claim_type".into() },
        ];
        let expected: [[&str; 11]; 4] = [
            [
                "uuid",
                "text",
                "citext",
                "int",
                "bigint",
                "boolean",
                "date",
                "timestamptz",
                "decimal(10,4)",
                "bytea",
                "intake.claim_type",
            ],
            [
                "uniqueidentifier",
                "nvarchar(max)",
                "nvarchar(320) COLLATE Latin1_General_CI_AS",
                "int",
                "bigint",
                "bit",
                "date",
                "datetimeoffset",
                "decimal(10,4)",
                "varbinary(max)",
                "nvarchar(32)",
            ],
            [
                "char(36)",
                "text",
                "varchar(320)",
                "int",
                "bigint",
                "tinyint(1)",
                "date",
                "timestamp",
                "decimal(10,4)",
                "blob",
                "enum('auto', 'property')",
            ],
            [
                "raw(16)",
                "clob",
                "varchar2(320)",
                "number(10)",
                "number(19)",
                "number(1)",
                "date",
                "timestamp with time zone",
                "number(10,4)",
                "blob",
                "varchar2(32)",
            ],
        ];
        for (d, want) in ALL.iter().zip(expected) {
            let got: Vec<String> = types.iter().map(|t| type_sql(*d, t, &m).unwrap()).collect();
            assert_eq!(got, want, "{d:?}");
        }
    }

    #[test]
    fn type_sql_unknown_enum_returns_unknown_enum_error() {
        for d in ALL {
            assert_eq!(
                type_sql(d, &FieldType::Enum { name: "nope".into() }, &Model::default()),
                Err(DdlError::UnknownEnum("nope".into()))
            );
        }
    }

    #[test]
    fn default_sql_each_default_kind_maps_per_dialect() {
        let got: Vec<[String; 3]> = ALL
            .iter()
            .map(|d| {
                [
                    default_sql(*d, &DefaultValue::GeneratedUuid),
                    default_sql(*d, &DefaultValue::Now),
                    default_sql(*d, &DefaultValue::Literal { sql: "'x'".into() }),
                ]
            })
            .collect();
        let want = [
            ["gen_random_uuid()", "now()", "'x'"],
            ["newsequentialid()", "sysdatetimeoffset()", "'x'"],
            ["(uuid())", "CURRENT_TIMESTAMP", "'x'"],
            ["sys_guid()", "systimestamp", "'x'"],
        ];
        assert_eq!(got, want.map(|r| r.map(String::from)));
    }

    #[test]
    fn column_sql_nullable_column_has_no_not_null() {
        let f = Field::new("bio", FieldType::Text).nullable();
        assert_eq!(column_sql(Dialect::PostgreSql, &f, &Model::default()).unwrap(), "bio text");
    }

    #[test]
    fn column_sql_primary_key_omits_not_null_and_adds_default() {
        let f = Field::new("id", FieldType::Uuid).pk();
        assert_eq!(
            column_sql(Dialect::SqlServer, &f, &Model::default()).unwrap(),
            "[id] uniqueidentifier PRIMARY KEY DEFAULT newsequentialid()"
        );
    }

    #[test]
    fn column_sql_reference_quotes_target_table_and_column() {
        let f = Field::new("user_id", FieldType::Uuid).nullable().references("app", "users", "id");
        assert_eq!(
            column_sql(Dialect::MySql, &f, &Model::default()).unwrap(),
            "`user_id` char(36) REFERENCES `app`.`users`(`id`)"
        );
    }

    #[test]
    fn create_enum_only_postgresql_creates_a_type() {
        let e = &v6().enums[0];
        assert_eq!(
            create_enum(Dialect::PostgreSql, e).as_deref(),
            Some("CREATE TYPE intake.claim_type AS ENUM ('auto', 'property');")
        );
        for d in [Dialect::SqlServer, Dialect::MySql, Dialect::Oracle] {
            assert_eq!(create_enum(d, e), None, "{d:?}");
        }
    }

    #[test]
    fn create_table_unknown_enum_returns_error() {
        let e = entity("t", vec![Field::new("k", FieldType::Enum { name: "missing".into() })]);
        assert_eq!(
            create_table(Dialect::PostgreSql, &e, &Model::default()),
            Err(DdlError::UnknownEnum("missing".into()))
        );
    }

    #[test]
    fn create_table_oracle_quotes_nothing_and_maps_types() {
        let e = entity(
            "t",
            vec![Field::new("n", FieldType::Int), Field::new("flag", FieldType::Bool).nullable()],
        );
        assert_eq!(
            create_table(Dialect::Oracle, &e, &Model::default()).unwrap(),
            "CREATE TABLE s.t (\n  n number(10) NOT NULL,\n  flag number(1)\n);"
        );
    }

    #[test]
    fn schema_sql_empty_model_is_a_single_newline() {
        for d in ALL {
            assert_eq!(schema_sql(d, &Model::default()).unwrap(), "\n");
        }
    }

    #[test]
    fn check_pii_every_hint_flagged_case_insensitively() {
        let names = [
            "FullName",
            "EMAIL",
            "phone_number",
            "home_address",
            "ssn",
            "birth_date",
            "dob",
            "passport_no",
        ];
        let mut m = Model::default();
        m.entities
            .push(entity("people", names.iter().map(|n| Field::new(n, FieldType::Text)).collect()));
        m.entities[0].fields.push(Field::new("total", FieldType::Int));
        let flagged: Vec<String> = check_pii(&m)
            .into_iter()
            .map(|e| match e {
                DdlError::UnclassifiedPii { entity, field } => format!("{entity}.{field}"),
                other => panic!("unexpected {other:?}"),
            })
            .collect();
        assert_eq!(flagged, names.map(|n| format!("people.{n}")));
    }

    #[test]
    fn check_pii_any_classification_clears_the_flag() {
        let mut m = Model::default();
        m.entities.push(entity(
            "people",
            vec![Field::new("email", FieldType::Text).classify(Classification::Public)],
        ));
        assert!(check_pii(&m).is_empty());
    }

    #[test]
    fn diff_identical_models_has_no_changes() {
        assert!(diff(&v7(), &v7()).is_empty());
    }

    #[test]
    fn diff_lists_new_enums_fields_and_entities_in_order() {
        let changes = diff(&v6(), &v7());
        let summary: Vec<String> = changes
            .iter()
            .map(|c| match c {
                Change::AddEnum(e) => format!("enum {}", e.name),
                Change::AddEntity(e) => format!("entity {}", e.table),
                Change::AddField { entity, field } => format!("field {}.{}", entity.table, field.name),
            })
            .collect();
        assert_eq!(summary, ["enum claim_status", "field claims.status", "entity claim_status_history"]);
    }

    #[test]
    fn diff_matches_entities_by_schema_and_table_not_name() {
        let mut renamed = v6();
        renamed.entities[0].name = "Renamed".into();
        assert!(diff(&v6(), &renamed).is_empty());
        let mut moved = v6();
        moved.entities[0].schema = "archive".into();
        assert!(matches!(diff(&v6(), &moved).as_slice(), [Change::AddEntity(e)] if e.schema == "archive"));
    }

    #[test]
    fn migration_no_changes_is_empty_up_and_down() {
        let m = migration(Dialect::MySql, &v7(), &v7()).unwrap();
        assert_eq!(
            m,
            Migration {
                up: "\n".into(),
                down: "\n".into(),
                rewrites_table: false
            }
        );
    }

    #[test]
    fn migration_required_column_without_default_rewrites_table() {
        let mut to = v6();
        to.entities[0].fields.push(Field::new("score", FieldType::Int));
        for d in ALL {
            assert!(migration(d, &v6(), &to).unwrap().rewrites_table, "{d:?}");
        }
    }

    #[test]
    fn migration_nullable_or_literal_default_column_does_not_rewrite() {
        let mut to = v6();
        to.entities[0].fields.push(Field::new("note", FieldType::Text).nullable());
        to.entities[0]
            .fields
            .push(Field::new("score", FieldType::Int).default(DefaultValue::Literal { sql: "0".into() }));
        for d in ALL {
            assert!(!migration(d, &v6(), &to).unwrap().rewrites_table, "{d:?}");
        }
    }

    #[test]
    fn migration_volatile_default_rewrites_except_on_postgresql() {
        let mut to = v6();
        to.entities[0]
            .fields
            .push(Field::new("seen_at", FieldType::Timestamptz).default(DefaultValue::Now));
        let got: Vec<bool> = ALL.iter().map(|d| migration(*d, &v6(), &to).unwrap().rewrites_table).collect();
        assert_eq!(got, [false, true, true, true]);
    }

    #[test]
    fn migration_add_column_keyword_per_dialect_and_matching_drop() {
        let mut to = v6();
        to.entities[0].fields.push(Field::new("note", FieldType::Text).nullable());
        let got: Vec<(String, String)> = ALL
            .iter()
            .map(|d| {
                let m = migration(*d, &v6(), &to).unwrap();
                (m.up, m.down)
            })
            .collect();
        assert_eq!(
            got,
            [
                (
                    "ALTER TABLE intake.claims ADD COLUMN note text;\n",
                    "ALTER TABLE intake.claims DROP COLUMN note;\n"
                ),
                (
                    "ALTER TABLE [intake].[claims] ADD [note] nvarchar(max);\n",
                    "ALTER TABLE [intake].[claims] DROP COLUMN [note];\n"
                ),
                (
                    "ALTER TABLE `intake`.`claims` ADD COLUMN `note` text;\n",
                    "ALTER TABLE `intake`.`claims` DROP COLUMN `note`;\n"
                ),
                (
                    "ALTER TABLE intake.claims ADD note clob;\n",
                    "ALTER TABLE intake.claims DROP COLUMN note;\n"
                ),
            ]
            .map(|(u, d)| (u.to_string(), d.to_string()))
        );
    }

    #[test]
    fn migration_new_enum_without_type_support_adds_nothing() {
        let mut to = v6();
        to.enums.push(EnumType {
            schema: "intake".into(),
            name: "unused".into(),
            values: vec!["a".into()],
        });
        for d in [Dialect::SqlServer, Dialect::MySql, Dialect::Oracle] {
            let m = migration(d, &v6(), &to).unwrap();
            assert_eq!((m.up.as_str(), m.down.as_str()), ("\n", "\n"), "{d:?}");
        }
    }

    #[test]
    fn migration_new_field_with_unknown_enum_returns_error() {
        let mut to = v6();
        to.entities[0]
            .fields
            .push(Field::new("k", FieldType::Enum { name: "ghost".into() }));
        assert_eq!(
            migration(Dialect::PostgreSql, &v6(), &to),
            Err(DdlError::UnknownEnum("ghost".into()))
        );
    }
}
