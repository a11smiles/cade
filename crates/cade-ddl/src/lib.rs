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
}
