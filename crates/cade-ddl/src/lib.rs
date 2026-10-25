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
    /// One SQL literal written as is: a number, `TRUE`/`FALSE`/`NULL`, or a quoted string
    /// with its quotes already doubled. Anything else is rejected as [`DdlError::UnsafeDefault`].
    Literal {
        sql: String,
    },
    /// A string default, quoted and escaped per dialect.
    Text {
        value: String,
    },
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
    #[error("the default for {field} isn't a single SQL literal: {sql}")]
    UnsafeDefault { field: String, sql: String },
}

/// Quote an identifier, doubling the dialect's closing quote inside it. PostgreSQL and
/// Oracle leave plain names (`[A-Za-z_][A-Za-z0-9_]*`) bare so their case folding is
/// unchanged, and double-quote anything else.
fn q(d: Dialect, ident: &str) -> String {
    match d {
        Dialect::PostgreSql | Dialect::Oracle if is_plain_ident(ident) => ident.to_string(),
        Dialect::PostgreSql | Dialect::Oracle => format!("\"{}\"", ident.replace('"', "\"\"")),
        Dialect::SqlServer => format!("[{}]", ident.replace(']', "]]")),
        Dialect::MySql => format!("`{}`", ident.replace('`', "``")),
    }
}

fn is_plain_ident(ident: &str) -> bool {
    let mut chars = ident.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// True when `sql` is exactly one literal: a number, `TRUE`/`FALSE`/`NULL`, or a quoted
/// string whose inner quotes are doubled (and, for MySQL, with no backslash escapes).
fn is_single_literal(d: Dialect, sql: &str) -> bool {
    let number = sql.strip_prefix('-').unwrap_or(sql);
    let (whole, frac) = number.split_once('.').unwrap_or((number, "0"));
    let is_digits = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit());
    if is_digits(whole) && is_digits(frac) {
        return true;
    }
    if ["TRUE", "FALSE", "NULL"].iter().any(|k| sql.eq_ignore_ascii_case(k)) {
        return true;
    }
    let Some(inner) = sql.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) else {
        return false;
    };
    if d == Dialect::MySql && inner.contains('\\') {
        return false;
    }
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c == '\'' && chars.next() != Some('\'') {
            return false;
        }
    }
    true
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
                MySql => format!("enum({})", enum_values(d, e)),
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
        (_, DefaultValue::Text { value }) => lit(d, value),
    }
}

fn column_sql(d: Dialect, f: &Field, model: &Model) -> Result<String, DdlError> {
    if let Some(DefaultValue::Literal { sql }) = &f.default
        && !is_single_literal(d, sql)
    {
        return Err(DdlError::UnsafeDefault {
            field: f.name.clone(),
            sql: sql.clone(),
        });
    }
    let mut s = format!("{} {}", q(d, &f.name), type_sql(d, &f.ty, model)?);
    let default = f.default.as_ref().map(|def| format!(" DEFAULT {}", default_sql(d, def)));
    // Oracle only accepts DEFAULT before inline constraints; the others keep it after.
    if d == Dialect::Oracle {
        s.push_str(default.as_deref().unwrap_or_default());
    }
    if f.primary_key {
        s.push_str(" PRIMARY KEY");
    } else if !f.nullable {
        s.push_str(" NOT NULL");
    }
    if d != Dialect::Oracle {
        s.push_str(default.as_deref().unwrap_or_default());
    }
    if let Some(r) = &f.references
        && d != Dialect::MySql
    {
        s.push_str(&format!(" REFERENCES {}({})", qualified(d, &r.schema, &r.table), q(d, &r.column)));
    }
    Ok(s)
}

/// A string literal with its quote doubled; MySQL also treats `\` as an escape, so it's doubled there too.
fn lit(d: Dialect, value: &str) -> String {
    let value = if d == Dialect::MySql {
        value.replace('\\', "\\\\")
    } else {
        value.to_string()
    };
    format!("'{}'", value.replace('\'', "''"))
}

fn enum_values(d: Dialect, e: &EnumType) -> String {
    e.values.iter().map(|v| lit(d, v)).collect::<Vec<_>>().join(", ")
}

/// The named CHECK that limits a SQL Server or Oracle enum column to its values.
/// PostgreSQL has enum types and MySQL inline `enum(...)`, so neither needs one.
fn check_sql(d: Dialect, table: &str, f: &Field, model: &Model) -> Result<Option<String>, DdlError> {
    let FieldType::Enum { name } = &f.ty else { return Ok(None) };
    if !matches!(d, Dialect::SqlServer | Dialect::Oracle) {
        return Ok(None);
    }
    let e = model
        .enums
        .iter()
        .find(|e| &e.name == name)
        .ok_or_else(|| DdlError::UnknownEnum(name.clone()))?;
    Ok(Some(format!(
        "CONSTRAINT {} CHECK ({} IN ({}))",
        q(d, &constraint_name(d, "ck", table, &f.name)),
        q(d, &f.name),
        enum_values(d, e)
    )))
}

/// A column with its inline constraints, as it appears in CREATE TABLE and ALTER TABLE … ADD.
fn column_def(d: Dialect, table: &str, f: &Field, model: &Model) -> Result<String, DdlError> {
    let column = column_sql(d, f, model)?;
    Ok(match check_sql(d, table, f, model)? {
        Some(check) => format!("{column} {check}"),
        None => column,
    })
}

/// Explicit constraint name `<prefix>_<table>_<column>`, cut to the dialect's identifier limit.
fn constraint_name(d: Dialect, prefix: &str, table: &str, column: &str) -> String {
    let max = if d == Dialect::MySql { 64 } else { 128 };
    format!("{prefix}_{table}_{column}").chars().take(max).collect()
}

/// MySQL's table-level foreign key for a referencing field. InnoDB ignores inline
/// `REFERENCES`, so MySQL needs this; the other dialects reference inline.
fn foreign_key_sql(d: Dialect, table: &str, f: &Field) -> Option<String> {
    let r = f.references.as_ref().filter(|_| d == Dialect::MySql)?;
    Some(format!(
        "CONSTRAINT {} FOREIGN KEY ({}) REFERENCES {}({})",
        q(d, &constraint_name(d, "fk", table, &f.name)),
        q(d, &f.name),
        qualified(d, &r.schema, &r.table),
        q(d, &r.column)
    ))
}

pub fn create_enum(d: Dialect, e: &EnumType) -> Option<String> {
    match d {
        Dialect::PostgreSql => Some(format!(
            "CREATE TYPE {} AS ENUM ({});",
            qualified(d, &e.schema, &e.name),
            enum_values(d, e)
        )),
        _ => None, // inline enum (MySQL) or check-constrained string (SQL Server, Oracle)
    }
}

pub fn create_table(d: Dialect, e: &Entity, model: &Model) -> Result<String, DdlError> {
    let mut lines = e
        .fields
        .iter()
        .map(|f| column_def(d, &e.table, f, model).map(|c| format!("  {c}")))
        .collect::<Result<Vec<_>, _>>()?;
    lines.extend(
        e.fields
            .iter()
            .filter_map(|f| foreign_key_sql(d, &e.table, f))
            .map(|fk| format!("  {fk}")),
    );
    Ok(format!(
        "CREATE TABLE {} (\n{}\n);",
        qualified(d, &e.schema, &e.table),
        lines.join(",\n")
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
                let table = qualified(d, &entity.schema, &entity.table);
                let column = column_def(d, &entity.table, &field, to)?;
                let name = q(d, &field.name);
                let constraint = |prefix| q(d, &constraint_name(d, prefix, &entity.table, &field.name));
                // One statement each way, so a column and its named constraint come and go together.
                // SQL Server won't drop a column that a CHECK still uses; Oracle drops single-column
                // constraints with the column.
                let fk = foreign_key_sql(d, &entity.table, &field);
                up.push(match &fk {
                    Some(fk) => format!("ALTER TABLE {table} {kw} {column}, ADD {fk};"),
                    None => format!("ALTER TABLE {table} {kw} {column};"),
                });
                down.push(if fk.is_some() {
                    format!("ALTER TABLE {table} DROP FOREIGN KEY {}, DROP COLUMN {name};", constraint("fk"))
                } else if d == Dialect::SqlServer && check_sql(d, &entity.table, &field, to)?.is_some() {
                    format!("ALTER TABLE {table} DROP CONSTRAINT {}, COLUMN {name};", constraint("ck"))
                } else {
                    format!("ALTER TABLE {table} DROP COLUMN {name};")
                });
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
        assert!(s.contains("changed_at timestamp with time zone DEFAULT systimestamp NOT NULL"));
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
            column_sql(Dialect::SqlServer, &f, &Model::default()).unwrap(),
            "[user_id] uniqueidentifier REFERENCES [app].[users]([id])"
        );
    }

    #[test]
    fn column_sql_mysql_reference_is_not_inline() {
        // InnoDB parses an inline REFERENCES and ignores it; MySQL gets a table-level FOREIGN KEY instead.
        let f = Field::new("user_id", FieldType::Uuid).nullable().references("app", "users", "id");
        assert_eq!(column_sql(Dialect::MySql, &f, &Model::default()).unwrap(), "`user_id` char(36)");
    }

    #[test]
    fn create_table_mysql_reference_emits_named_table_level_foreign_key() {
        let e = entity(
            "orders",
            vec![Field::new("customer_id", FieldType::Uuid).references("s", "customers", "id")],
        );
        assert_eq!(
            create_table(Dialect::MySql, &e, &Model::default()).unwrap(),
            "CREATE TABLE `s`.`orders` (\n  `customer_id` char(36) NOT NULL,\n  \
             CONSTRAINT `fk_orders_customer_id` FOREIGN KEY (`customer_id`) REFERENCES `s`.`customers`(`id`)\n);"
        );
    }

    #[test]
    fn create_table_mysql_long_foreign_key_name_is_cut_to_64_chars() {
        let table = "t".repeat(60);
        let e = entity(
            &table,
            vec![Field::new("owner_id", FieldType::Uuid).references("s", "owners", "id")],
        );
        let sql = create_table(Dialect::MySql, &e, &Model::default()).unwrap();
        let name = sql.split("CONSTRAINT `").nth(1).and_then(|r| r.split('`').next()).unwrap();
        assert_eq!(name, format!("fk_{}_", "t".repeat(60)));
        assert_eq!(name.chars().count(), 64);
    }

    const HOSTILE: &str = "a]b\"c`d";

    #[test]
    fn q_hostile_identifier_escapes_quote_per_dialect() {
        let got: Vec<String> = ALL.iter().map(|d| q(*d, HOSTILE)).collect();
        assert_eq!(got, ["\"a]b\"\"c`d\"", "[a]]b\"c`d]", "`a]b\"c``d`", "\"a]b\"\"c`d\""]);
    }

    #[test]
    fn q_plain_identifier_with_capitals_and_digits_stays_unquoted() {
        assert_eq!(q(Dialect::PostgreSql, "Order_2"), "Order_2");
        assert_eq!(q(Dialect::Oracle, "_tmp1"), "_tmp1");
    }

    #[test]
    fn create_enum_value_with_quote_is_escaped() {
        let e = EnumType {
            schema: "s".into(),
            name: "kind".into(),
            values: vec!["it's".into()],
        };
        assert_eq!(
            create_enum(Dialect::PostgreSql, &e).as_deref(),
            Some("CREATE TYPE s.kind AS ENUM ('it''s');")
        );
    }

    #[test]
    fn type_sql_mysql_enum_value_with_quote_and_backslash_is_escaped() {
        let m = Model {
            enums: vec![EnumType {
                schema: "s".into(),
                name: "kind".into(),
                values: vec!["it's".into(), "a\\".into()],
            }],
            entities: vec![],
        };
        assert_eq!(
            type_sql(Dialect::MySql, &FieldType::Enum { name: "kind".into() }, &m).unwrap(),
            "enum('it''s', 'a\\\\')"
        );
    }

    #[test]
    fn column_sql_text_default_with_quote_is_escaped_per_dialect() {
        let f = Field::new("c", FieldType::CiText).default(DefaultValue::Text { value: "a'b\\c".into() });
        let got: Vec<String> = ALL.iter().map(|d| column_sql(*d, &f, &Model::default()).unwrap()).collect();
        assert_eq!(
            got,
            [
                "c citext NOT NULL DEFAULT 'a''b\\c'",
                "[c] nvarchar(320) COLLATE Latin1_General_CI_AS NOT NULL DEFAULT 'a''b\\c'",
                "`c` varchar(320) NOT NULL DEFAULT 'a''b\\\\c'",
                "c varchar2(320) DEFAULT 'a''b\\c' NOT NULL",
            ]
        );
    }

    #[test]
    fn column_sql_literal_default_with_injection_returns_unsafe_default() {
        for sql in ["'x'); DROP TABLE t; --", "0; DROP TABLE t", "'it's'", "now()", "'a\\'"] {
            let f = Field::new("c", FieldType::Text).default(DefaultValue::Literal { sql: sql.into() });
            assert_eq!(
                column_sql(Dialect::MySql, &f, &Model::default()),
                Err(DdlError::UnsafeDefault {
                    field: "c".into(),
                    sql: sql.into()
                }),
                "{sql}"
            );
        }
    }

    #[test]
    fn column_sql_literal_default_single_literal_is_written_as_is() {
        for sql in ["0", "-1.5", "'it''s'", "TRUE", "false", "NULL", "''"] {
            let f = Field::new("c", FieldType::Text).default(DefaultValue::Literal { sql: sql.into() });
            for d in ALL {
                assert_eq!(
                    column_sql(d, &f, &Model::default())
                        .unwrap()
                        .split(" DEFAULT ")
                        .nth(1)
                        .map(|s| s.trim_end_matches(" NOT NULL")),
                    Some(sql),
                    "{d:?} {sql}"
                );
            }
        }
    }

    #[test]
    fn create_table_hostile_table_name_and_default_are_escaped_per_dialect() {
        let e = entity(
            HOSTILE,
            vec![
                Field::new("c", FieldType::CiText)
                    .nullable()
                    .default(DefaultValue::Text { value: "x'y".into() }),
            ],
        );
        let got: Vec<String> = ALL.iter().map(|d| create_table(*d, &e, &Model::default()).unwrap()).collect();
        assert_eq!(
            got,
            [
                "CREATE TABLE s.\"a]b\"\"c`d\" (\n  c citext DEFAULT 'x''y'\n);",
                "CREATE TABLE [s].[a]]b\"c`d] (\n  [c] nvarchar(320) COLLATE Latin1_General_CI_AS DEFAULT 'x''y'\n);",
                "CREATE TABLE `s`.`a]b\"c``d` (\n  `c` varchar(320) DEFAULT 'x''y'\n);",
                "CREATE TABLE s.\"a]b\"\"c`d\" (\n  c varchar2(320) DEFAULT 'x''y'\n);",
            ]
        );
    }

    fn kind_model(values: &[&str]) -> Model {
        Model {
            enums: vec![EnumType {
                schema: "s".into(),
                name: "kind".into(),
                values: values.iter().map(|v| v.to_string()).collect(),
            }],
            entities: vec![entity("t", vec![Field::new("id", FieldType::Uuid).pk()])],
        }
    }

    fn kind_field() -> Field {
        Field::new("k", FieldType::Enum { name: "kind".into() })
    }

    #[test]
    fn create_table_sqlserver_enum_adds_named_check_with_values() {
        let m = kind_model(&["a", "b"]);
        assert_eq!(
            create_table(Dialect::SqlServer, &entity("t", vec![kind_field()]), &m).unwrap(),
            "CREATE TABLE [s].[t] (\n  [k] nvarchar(32) NOT NULL CONSTRAINT [ck_t_k] CHECK ([k] IN ('a', 'b'))\n);"
        );
    }

    #[test]
    fn create_table_oracle_enum_adds_named_check_with_values() {
        let m = kind_model(&["a", "b"]);
        assert_eq!(
            create_table(Dialect::Oracle, &entity("t", vec![kind_field()]), &m).unwrap(),
            "CREATE TABLE s.t (\n  k varchar2(32) NOT NULL CONSTRAINT ck_t_k CHECK (k IN ('a', 'b'))\n);"
        );
    }

    #[test]
    fn create_table_enum_value_with_quote_is_escaped_in_check() {
        let m = kind_model(&["it's"]);
        let e = entity("t", vec![kind_field()]);
        assert!(
            create_table(Dialect::SqlServer, &e, &m)
                .unwrap()
                .contains("CHECK ([k] IN ('it''s'))")
        );
        assert!(create_table(Dialect::Oracle, &e, &m).unwrap().contains("CHECK (k IN ('it''s'))"));
    }

    #[test]
    fn create_table_postgresql_and_mysql_enums_have_no_check() {
        let m = kind_model(&["a"]);
        let e = entity("t", vec![kind_field()]);
        for d in [Dialect::PostgreSql, Dialect::MySql] {
            assert!(!create_table(d, &e, &m).unwrap().contains("CHECK"), "{d:?}");
        }
    }

    #[test]
    fn migration_add_enum_field_sqlserver_drops_check_with_column() {
        let from = kind_model(&["a"]);
        let mut to = from.clone();
        to.entities[0].fields.push(kind_field().nullable());
        let m = migration(Dialect::SqlServer, &from, &to).unwrap();
        assert_eq!(
            m.up,
            "ALTER TABLE [s].[t] ADD [k] nvarchar(32) CONSTRAINT [ck_t_k] CHECK ([k] IN ('a'));\n"
        );
        assert_eq!(m.down, "ALTER TABLE [s].[t] DROP CONSTRAINT [ck_t_k], COLUMN [k];\n");
    }

    #[test]
    fn migration_add_enum_field_oracle_adds_check_and_drops_it_with_column() {
        let from = kind_model(&["a"]);
        let mut to = from.clone();
        to.entities[0].fields.push(kind_field().nullable());
        let m = migration(Dialect::Oracle, &from, &to).unwrap();
        assert_eq!(m.up, "ALTER TABLE s.t ADD k varchar2(32) CONSTRAINT ck_t_k CHECK (k IN ('a'));\n");
        // Oracle drops a column's single-column constraints with it.
        assert_eq!(m.down, "ALTER TABLE s.t DROP COLUMN k;\n");
    }

    #[test]
    fn migration_add_reference_field_mysql_adds_and_drops_named_foreign_key() {
        let from = Model {
            enums: vec![],
            entities: vec![entity("t", vec![Field::new("id", FieldType::Uuid).pk()])],
        };
        let mut to = from.clone();
        to.entities[0]
            .fields
            .push(Field::new("owner_id", FieldType::Uuid).nullable().references("s", "owners", "id"));
        let m = migration(Dialect::MySql, &from, &to).unwrap();
        assert_eq!(
            m.up,
            "ALTER TABLE `s`.`t` ADD COLUMN `owner_id` char(36), \
             ADD CONSTRAINT `fk_t_owner_id` FOREIGN KEY (`owner_id`) REFERENCES `s`.`owners`(`id`);\n"
        );
        assert_eq!(
            m.down,
            "ALTER TABLE `s`.`t` DROP FOREIGN KEY `fk_t_owner_id`, DROP COLUMN `owner_id`;\n"
        );
    }

    #[test]
    fn column_sql_oracle_default_precedes_primary_key() {
        let f = Field::new("id", FieldType::Uuid).pk();
        assert_eq!(
            column_sql(Dialect::Oracle, &f, &Model::default()).unwrap(),
            "id raw(16) DEFAULT sys_guid() PRIMARY KEY"
        );
    }

    #[test]
    fn column_sql_oracle_default_precedes_not_null_and_references() {
        let f = Field::new("rank", FieldType::Int)
            .default(DefaultValue::Literal { sql: "0".into() })
            .references("s", "ranks", "id");
        assert_eq!(
            column_sql(Dialect::Oracle, &f, &Model::default()).unwrap(),
            "rank number(10) DEFAULT 0 NOT NULL REFERENCES s.ranks(id)"
        );
    }

    #[test]
    fn column_sql_other_dialects_keep_default_after_not_null() {
        let f = Field::new("rank", FieldType::Int).default(DefaultValue::Literal { sql: "0".into() });
        let got: Vec<String> = [Dialect::PostgreSql, Dialect::SqlServer, Dialect::MySql]
            .iter()
            .map(|d| column_sql(*d, &f, &Model::default()).unwrap())
            .collect();
        assert_eq!(
            got,
            [
                "rank int NOT NULL DEFAULT 0",
                "[rank] int NOT NULL DEFAULT 0",
                "`rank` int NOT NULL DEFAULT 0"
            ]
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
