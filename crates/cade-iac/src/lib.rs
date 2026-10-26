//! Azure infrastructure as code from an Architecture project.
//!
//! The canvas holds a [`Design`] (resources plus containment); this crate
//! emits Bicep or Terraform (azurerm). 1.0 is Azure only. The resource set
//! here is the slice shown in the designs; the full catalog is generated
//! from Bicep type schemas (ARCH backlog).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Target {
    Bicep,
    Terraform,
}

/// Subnet references (`subnet`, `private_endpoint_subnet`) are subnet resource
/// IDs (`/subscriptions/…/subnets/<name>`), emitted as string literals.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Resource {
    /// A Linux web app on its own App Service plan of `plan_sku`.
    #[serde(rename = "Microsoft.Web/sites")]
    AppService {
        name: String,
        plan_sku: String,
        public_access: bool,
        system_identity: bool,
        /// Subnet for regional VNet integration.
        subnet: Option<String>,
        settings: Vec<(String, String)>,
    },
    #[serde(rename = "Microsoft.Sql/servers")]
    SqlServer { name: String, database: String, sku: String },
    #[serde(rename = "Microsoft.KeyVault/vaults")]
    KeyVault {
        name: String,
        /// Subnet for a private endpoint to the vault.
        private_endpoint_subnet: Option<String>,
    },
    #[serde(rename = "Microsoft.Storage/storageAccounts")]
    Storage { name: String, redundancy: String },
}

impl Resource {
    pub fn name(&self) -> &str {
        match self {
            Resource::AppService { name, .. }
            | Resource::SqlServer { name, .. }
            | Resource::KeyVault { name, .. }
            | Resource::Storage { name, .. } => name,
        }
    }

    /// Bicep symbol name derived from the resource name: lower camel case,
    /// with every character that can't appear in a symbol starting a new word.
    fn symbol(&self) -> String {
        let mut out = String::new();
        let mut upper = false;
        for ch in self.name().chars() {
            if !ch.is_ascii_alphanumeric() {
                upper = true;
            } else if out.is_empty() {
                if ch.is_ascii_digit() {
                    out.push('r');
                }
                out.push(ch.to_ascii_lowercase());
                upper = false;
            } else if upper {
                out.push(ch.to_ascii_uppercase());
                upper = false;
            } else {
                out.push(ch);
            }
        }
        out
    }

    /// Terraform resource label derived from the resource name: every character
    /// that can't appear in an identifier becomes `_`.
    fn label(&self) -> String {
        let id: String = self
            .name()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
            .collect();
        if id.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
            id
        } else {
            format!("_{id}")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Design {
    pub location: String,
    pub resource_group: String,
    pub resources: Vec<Resource>,
}

/// A Bicep single-quoted string literal with `\`, `'`, `${` and control characters escaped.
fn bq(s: &str) -> String {
    let mut out = String::from("'");
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\'' => out.push_str("\\'"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '$' if chars.peek() == Some(&'{') => out.push_str("\\$"),
            c if c.is_control() => out.push_str(&format!("\\u{{{:X}}}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('\'');
    out
}

/// A Terraform (HCL) quoted string with `\`, `"`, control characters and the
/// `${` / `%{` template sequences escaped.
fn tq(s: &str) -> String {
    let mut out = String::from("\"");
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '$' | '%' if chars.peek() == Some(&'{') => {
                out.push(c);
                out.push(c);
            }
            c if c.is_control() => out.push_str(&format!("\\u{:04X}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// A Terraform map key: bare when it's a plain identifier, quoted otherwise.
fn tf_key(k: &str) -> String {
    let plain = k.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        && k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    if plain { k.to_string() } else { tq(k) }
}

fn bicep_resource(r: &Resource) -> String {
    let sym = r.symbol();
    match r {
        Resource::AppService {
            name,
            plan_sku,
            public_access,
            system_identity,
            subnet,
            settings,
        } => {
            let mut s = format!(
                "resource {sym}Plan 'Microsoft.Web/serverfarms@2023-12-01' = {{\n  name: {}\n  location: location\n  kind: 'linux'\n  sku: {{ name: {} }}\n  properties: {{\n    reserved: true\n  }}\n}}\n\n",
                bq(&format!("plan-{name}")),
                bq(plan_sku)
            );
            s.push_str(&format!(
                "resource {sym} 'Microsoft.Web/sites@2023-12-01' = {{\n  name: {}\n  location: location\n  kind: 'app,linux'\n",
                bq(name)
            ));
            if *system_identity {
                s.push_str("  identity: { type: 'SystemAssigned' }\n");
            }
            s.push_str(&format!("  properties: {{\n    serverFarmId: {sym}Plan.id\n    httpsOnly: true\n"));
            s.push_str(&format!(
                "    publicNetworkAccess: '{}'\n",
                if *public_access { "Enabled" } else { "Disabled" }
            ));
            if let Some(sn) = subnet {
                s.push_str(&format!("    virtualNetworkSubnetId: {}\n", bq(sn)));
            }
            if !settings.is_empty() {
                s.push_str("    siteConfig: {\n      appSettings: [\n");
                for (k, v) in settings {
                    s.push_str(&format!("        {{ name: {}, value: {} }}\n", bq(k), bq(v)));
                }
                s.push_str("      ]\n    }\n");
            }
            s.push_str("  }\n}\n");
            s
        }
        Resource::SqlServer { name, database, sku } => format!(
            "resource {sym} 'Microsoft.Sql/servers@2023-08-01' = {{\n  name: {}\n  location: location\n  properties: {{\n    publicNetworkAccess: 'Disabled'\n    minimalTlsVersion: '1.2'\n  }}\n}}\n\nresource {sym}Db 'Microsoft.Sql/servers/databases@2023-08-01' = {{\n  parent: {sym}\n  name: {}\n  location: location\n  sku: {{ name: {} }}\n}}\n",
            bq(name),
            bq(database),
            bq(sku)
        ),
        Resource::KeyVault {
            name,
            private_endpoint_subnet,
        } => {
            let mut s = format!(
                "resource {sym} 'Microsoft.KeyVault/vaults@2023-07-01' = {{\n  name: {}\n  location: location\n  properties: {{\n    tenantId: subscription().tenantId\n    sku: {{ family: 'A', name: 'standard' }}\n    enableRbacAuthorization: true\n    publicNetworkAccess: 'Disabled'\n  }}\n}}\n",
                bq(name)
            );
            if let Some(sn) = private_endpoint_subnet {
                s.push_str(&format!(
                    "\nresource {sym}Pe 'Microsoft.Network/privateEndpoints@2023-11-01' = {{\n  name: {}\n  location: location\n  properties: {{\n    subnet: {{ id: {} }}\n    privateLinkServiceConnections: [\n      {{\n        name: {}\n        properties: {{\n          privateLinkServiceId: {sym}.id\n          groupIds: [ 'vault' ]\n        }}\n      }}\n    ]\n  }}\n}}\n",
                    bq(&format!("pe-{name}")),
                    bq(sn),
                    bq(name)
                ));
                s.push_str(&format!(
                    "\nresource {sym}PeDns 'Microsoft.Network/privateEndpoints/privateDnsZoneGroups@2023-11-01' = {{\n  parent: {sym}Pe\n  name: 'default'\n  properties: {{\n    privateDnsZoneConfigs: [\n      {{\n        name: 'privatelink-vaultcore-azure-net'\n        properties: {{\n          privateDnsZoneId: vaultDnsZone.id\n        }}\n      }}\n    ]\n  }}\n}}\n"
                ));
            }
            s
        }
        Resource::Storage { name, redundancy } => format!(
            "resource {sym} 'Microsoft.Storage/storageAccounts@2023-05-01' = {{\n  name: {}\n  location: location\n  kind: 'StorageV2'\n  sku: {{ name: {} }}\n  properties: {{\n    allowBlobPublicAccess: false\n    minimumTlsVersion: 'TLS1_2'\n  }}\n}}\n",
            bq(name),
            bq(&format!("Standard_{redundancy}"))
        ),
    }
}

fn tf_resource(r: &Resource) -> String {
    let id = r.label();
    match r {
        Resource::AppService {
            name,
            plan_sku,
            public_access,
            system_identity,
            subnet,
            settings,
        } => {
            let mut s = format!(
                "resource \"azurerm_service_plan\" \"{id}\" {{\n  name                = {}\n  location            = var.location\n  resource_group_name = var.resource_group_name\n  os_type             = \"Linux\"\n  sku_name            = {}\n}}\n\n",
                tq(&format!("plan-{name}")),
                tq(plan_sku)
            );
            s.push_str(&format!(
                "resource \"azurerm_linux_web_app\" \"{id}\" {{\n  name                = {}\n  location            = var.location\n  resource_group_name = var.resource_group_name\n  service_plan_id     = azurerm_service_plan.{id}.id\n  https_only          = true\n  public_network_access_enabled = {public_access}\n",
                tq(name)
            ));
            if let Some(sn) = subnet {
                s.push_str(&format!("  virtual_network_subnet_id = {}\n", tq(sn)));
            }
            if *system_identity {
                s.push_str("\n  identity {\n    type = \"SystemAssigned\"\n  }\n");
            }
            s.push_str("\n  site_config {}\n");
            if !settings.is_empty() {
                s.push_str("\n  app_settings = {\n");
                for (k, v) in settings {
                    s.push_str(&format!("    {} = {}\n", tf_key(k), tq(v)));
                }
                s.push_str("  }\n");
            }
            s.push_str("}\n");
            s
        }
        Resource::SqlServer { name, database, sku } => format!(
            "resource \"azurerm_mssql_server\" \"{id}\" {{\n  name                          = {}\n  location                      = var.location\n  resource_group_name           = var.resource_group_name\n  version                       = \"12.0\"\n  minimum_tls_version           = \"1.2\"\n  public_network_access_enabled = false\n}}\n\nresource \"azurerm_mssql_database\" \"{id}_db\" {{\n  name      = {}\n  server_id = azurerm_mssql_server.{id}.id\n  sku_name  = {}\n}}\n",
            tq(name),
            tq(database),
            tq(sku)
        ),
        Resource::KeyVault {
            name,
            private_endpoint_subnet,
        } => {
            let mut s = format!(
                "resource \"azurerm_key_vault\" \"{id}\" {{\n  name                          = {}\n  location                      = var.location\n  resource_group_name           = var.resource_group_name\n  tenant_id                     = data.azurerm_client_config.current.tenant_id\n  sku_name                      = \"standard\"\n  rbac_authorization_enabled    = true\n  public_network_access_enabled = false\n}}\n",
                tq(name)
            );
            if let Some(sn) = private_endpoint_subnet {
                s.push_str(&format!(
                    "\nresource \"azurerm_private_endpoint\" \"{id}\" {{\n  name                = {}\n  location            = var.location\n  resource_group_name = var.resource_group_name\n  subnet_id           = {}\n\n  private_service_connection {{\n    name                           = {}\n    private_connection_resource_id = azurerm_key_vault.{id}.id\n    subresource_names              = [\"vault\"]\n    is_manual_connection           = false\n  }}\n\n  private_dns_zone_group {{\n    name                 = \"default\"\n    private_dns_zone_ids = [azurerm_private_dns_zone.vault.id]\n  }}\n}}\n",
                    tq(&format!("pe-{name}")),
                    tq(sn),
                    tq(name)
                ));
            }
            s
        }
        Resource::Storage { name, redundancy } => format!(
            "resource \"azurerm_storage_account\" \"{id}\" {{\n  name                     = {}\n  location                 = var.location\n  resource_group_name      = var.resource_group_name\n  account_tier             = \"Standard\"\n  account_replication_type = {}\n  min_tls_version          = \"TLS1_2\"\n  allow_nested_items_to_be_public = false\n}}\n",
            tq(name),
            tq(redundancy)
        ),
    }
}

/// The virtual network a subnet resource ID belongs to: the ID up to `/subnets/`
/// (matched without case, as Azure resource IDs are).
fn vnet_id(subnet: &str) -> Option<&str> {
    let at = subnet.to_ascii_lowercase().rfind("/subnets/")?;
    (at > 0).then(|| &subnet[..at])
}

/// The `privatelink.vaultcore.azure.net` zone and one link per virtual network
/// that holds a Key Vault private endpoint, so vault names resolve privately.
/// Empty when no vault has a private endpoint.
fn vault_dns(design: &Design, target: Target) -> String {
    let subnets: Vec<&str> = design
        .resources
        .iter()
        .filter_map(|r| match r {
            Resource::KeyVault {
                private_endpoint_subnet: Some(sn),
                ..
            } => Some(sn.as_str()),
            _ => None,
        })
        .collect();
    if subnets.is_empty() {
        return String::new();
    }
    let mut vnets: Vec<&str> = Vec::new();
    for v in subnets.iter().filter_map(|sn| vnet_id(sn)) {
        if !vnets.iter().any(|seen| seen.eq_ignore_ascii_case(v)) {
            vnets.push(v);
        }
    }
    let mut names: Vec<String> = Vec::new();
    for v in &vnets {
        let base = format!("link-{}", v.rsplit('/').next().unwrap_or(v));
        let taken = names.iter().filter(|n| **n == base || n.starts_with(&format!("{base}-"))).count();
        names.push(if taken == 0 { base } else { format!("{base}-{}", taken + 1) });
    }

    let mut s = match target {
        Target::Bicep => "resource vaultDnsZone 'Microsoft.Network/privateDnsZones@2024-06-01' = {\n  name: 'privatelink.vaultcore.azure.net'\n  location: 'global'\n}\n\n".to_string(),
        Target::Terraform => "resource \"azurerm_private_dns_zone\" \"vault\" {\n  name                = \"privatelink.vaultcore.azure.net\"\n  resource_group_name = var.resource_group_name\n}\n\n".to_string(),
    };
    for (i, (v, name)) in vnets.iter().zip(&names).enumerate() {
        let n = i + 1;
        s.push_str(&match target {
            Target::Bicep => format!(
                "resource vaultDnsLink{n} 'Microsoft.Network/privateDnsZones/virtualNetworkLinks@2024-06-01' = {{\n  parent: vaultDnsZone\n  name: {}\n  location: 'global'\n  properties: {{\n    registrationEnabled: false\n    virtualNetwork: {{ id: {} }}\n  }}\n}}\n\n",
                bq(name),
                bq(v)
            ),
            Target::Terraform => format!(
                "resource \"azurerm_private_dns_zone_virtual_network_link\" \"vault_{n}\" {{\n  name                  = {}\n  resource_group_name   = var.resource_group_name\n  private_dns_zone_name = azurerm_private_dns_zone.vault.name\n  virtual_network_id    = {}\n  registration_enabled  = false\n}}\n\n",
                tq(name),
                tq(v)
            ),
        });
    }
    s
}

/// The whole design for `target`. Bicep deploys at resource-group scope (the
/// group is recorded as metadata); Terraform takes the group and location as
/// variables that default to the design's values.
pub fn generate(design: &Design, target: Target) -> String {
    let (mut s, emit): (String, fn(&Resource) -> String) = match target {
        Target::Bicep => (
            format!(
                "// Generated by CADE from the Architecture canvas. Edits here sync back to the canvas.\ntargetScope = 'resourceGroup'\nmetadata resourceGroup = {}\n\nparam location string = {}\n\n",
                bq(&design.resource_group),
                bq(&design.location)
            ),
            bicep_resource,
        ),
        Target::Terraform => (
            format!(
                "# Generated by CADE from the Architecture canvas. Edits here sync back to the canvas.\nterraform {{\n  required_providers {{\n    azurerm = {{\n      source  = \"hashicorp/azurerm\"\n      version = \"~> 4.0\"\n    }}\n  }}\n}}\n\nprovider \"azurerm\" {{\n  features {{}}\n}}\n\nvariable \"location\" {{\n  type    = string\n  default = {}\n}}\n\nvariable \"resource_group_name\" {{\n  type    = string\n  default = {}\n}}\n\ndata \"azurerm_client_config\" \"current\" {{}}\n\n",
                tq(&design.location),
                tq(&design.resource_group)
            ),
            tf_resource,
        ),
    };
    for r in &design.resources {
        s.push_str(&emit(r));
        s.push('\n');
    }
    s.push_str(&vault_dns(design, target));
    s
}

/// Settings an app reads that the design does not define (System view check).
pub fn missing_settings(design: &Design, app_reads: &[&str]) -> Vec<String> {
    let defined: Vec<&str> = design
        .resources
        .iter()
        .filter_map(|r| match r {
            Resource::AppService { settings, .. } => Some(settings.iter().map(|(k, _)| k.as_str()).collect::<Vec<_>>()),
            _ => None,
        })
        .flatten()
        .collect();
    app_reads.iter().filter(|k| !defined.contains(k)).map(|k| k.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claims() -> Design {
        Design {
            location: "eastus2".into(),
            resource_group: "rg-claims-prod-eus2".into(),
            resources: vec![
                Resource::AppService {
                    name: "app-claims-prod".into(),
                    plan_sku: "P1v3".into(),
                    public_access: false,
                    system_identity: true,
                    subnet: Some(SUBNET.into()),
                    settings: vec![("DB_NAME".into(), "claims".into())],
                },
                Resource::SqlServer {
                    name: "sql-claims".into(),
                    database: "claims".into(),
                    sku: "GP_S_Gen5_2".into(),
                },
                Resource::KeyVault {
                    name: "kv-claims".into(),
                    private_endpoint_subnet: Some(SUBNET.into()),
                },
                Resource::Storage {
                    name: "stclaimsphotos".into(),
                    redundancy: "ZRS".into(),
                },
            ],
        }
    }

    #[test]
    fn bicep_locks_down_the_app() {
        let b = generate(&claims(), Target::Bicep);
        assert!(b.contains("resource appClaimsProd 'Microsoft.Web/sites@2023-12-01'"));
        assert!(b.contains("publicNetworkAccess: 'Disabled'"));
        assert!(b.contains("identity: { type: 'SystemAssigned' }"));
        assert!(b.contains("enableRbacAuthorization: true"));
    }

    #[test]
    fn terraform_uses_azurerm() {
        let t = generate(&claims(), Target::Terraform);
        assert!(t.contains("resource \"azurerm_linux_web_app\" \"app_claims_prod\""));
        assert!(t.contains("public_network_access_enabled = false"));
        assert!(t.contains("account_replication_type = \"ZRS\""));
    }

    #[test]
    fn finds_the_missing_photo_container_setting() {
        assert_eq!(
            missing_settings(&claims(), &["DB_NAME", "PHOTO_CONTAINER"]),
            vec!["PHOTO_CONTAINER"]
        );
    }

    fn app(public_access: bool, system_identity: bool, subnet: Option<&str>, settings: &[(&str, &str)]) -> Resource {
        Resource::AppService {
            name: "app-web".into(),
            plan_sku: "B1".into(),
            public_access,
            system_identity,
            subnet: subnet.map(String::from),
            settings: settings.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        }
    }

    fn design(resources: Vec<Resource>) -> Design {
        Design {
            location: "westus3".into(),
            resource_group: "rg-test".into(),
            resources,
        }
    }

    #[test]
    fn name_each_resource_kind_returns_its_name() {
        let names: Vec<String> = claims().resources.iter().map(|r| r.name().to_string()).collect();
        assert_eq!(names, ["app-claims-prod", "sql-claims", "kv-claims", "stclaimsphotos"]);
    }

    #[test]
    fn symbol_dashes_and_underscores_become_lower_camel_case() {
        let cases = [
            ("app-claims-prod", "appClaimsProd"),
            ("kv_claims_eu", "kvClaimsEu"),
            ("Storage-Acct", "storageAcct"),
            ("stclaimsphotos", "stclaimsphotos"),
            ("a--b", "aB"),
        ];
        for (name, want) in cases {
            let r = Resource::Storage {
                name: name.into(),
                redundancy: "LRS".into(),
            };
            assert_eq!(r.symbol(), want, "name {name}");
        }
    }

    #[test]
    fn bicep_resource_minimal_public_app_omits_identity_subnet_and_settings() {
        assert_eq!(
            bicep_resource(&app(true, false, None, &[])),
            "resource appWebPlan 'Microsoft.Web/serverfarms@2023-12-01' = {\n  name: 'plan-app-web'\n  location: location\n  kind: 'linux'\n  sku: { name: 'B1' }\n  properties: {\n    reserved: true\n  }\n}\n\nresource appWeb 'Microsoft.Web/sites@2023-12-01' = {\n  name: 'app-web'\n  location: location\n  kind: 'app,linux'\n  properties: {\n    serverFarmId: appWebPlan.id\n    httpsOnly: true\n    publicNetworkAccess: 'Enabled'\n  }\n}\n"
        );
    }

    #[test]
    fn bicep_resource_app_settings_listed_in_order() {
        let b = bicep_resource(&app(false, false, None, &[("A", "1"), ("B", "2")]));
        assert!(
            b.contains(
                "    siteConfig: {\n      appSettings: [\n        { name: 'A', value: '1' }\n        { name: 'B', value: '2' }\n      ]\n    }\n"
            ),
            "{b}"
        );
    }

    #[test]
    fn bicep_resource_sql_server_emits_server_and_child_database() {
        let r = Resource::SqlServer {
            name: "sql-x".into(),
            database: "db1".into(),
            sku: "S0".into(),
        };
        assert_eq!(
            bicep_resource(&r),
            "resource sqlX 'Microsoft.Sql/servers@2023-08-01' = {\n  name: 'sql-x'\n  location: location\n  properties: {\n    publicNetworkAccess: 'Disabled'\n    minimalTlsVersion: '1.2'\n  }\n}\n\nresource sqlXDb 'Microsoft.Sql/servers/databases@2023-08-01' = {\n  parent: sqlX\n  name: 'db1'\n  location: location\n  sku: { name: 'S0' }\n}\n"
        );
    }

    #[test]
    fn bicep_resource_key_vault_is_private_with_rbac() {
        let r = Resource::KeyVault {
            name: "kv-x".into(),
            private_endpoint_subnet: None,
        };
        assert_eq!(
            bicep_resource(&r),
            "resource kvX 'Microsoft.KeyVault/vaults@2023-07-01' = {\n  name: 'kv-x'\n  location: location\n  properties: {\n    tenantId: subscription().tenantId\n    sku: { family: 'A', name: 'standard' }\n    enableRbacAuthorization: true\n    publicNetworkAccess: 'Disabled'\n  }\n}\n"
        );
    }

    #[test]
    fn bicep_resource_storage_prefixes_redundancy_with_standard() {
        let r = Resource::Storage {
            name: "stx".into(),
            redundancy: "GRS".into(),
        };
        assert_eq!(
            bicep_resource(&r),
            "resource stx 'Microsoft.Storage/storageAccounts@2023-05-01' = {\n  name: 'stx'\n  location: location\n  kind: 'StorageV2'\n  sku: { name: 'Standard_GRS' }\n  properties: {\n    allowBlobPublicAccess: false\n    minimumTlsVersion: 'TLS1_2'\n  }\n}\n"
        );
    }

    #[test]
    fn tf_resource_minimal_public_app_omits_identity_and_settings() {
        assert_eq!(
            tf_resource(&app(true, false, None, &[])),
            "resource \"azurerm_service_plan\" \"app_web\" {\n  name                = \"plan-app-web\"\n  location            = var.location\n  resource_group_name = var.resource_group_name\n  os_type             = \"Linux\"\n  sku_name            = \"B1\"\n}\n\nresource \"azurerm_linux_web_app\" \"app_web\" {\n  name                = \"app-web\"\n  location            = var.location\n  resource_group_name = var.resource_group_name\n  service_plan_id     = azurerm_service_plan.app_web.id\n  https_only          = true\n  public_network_access_enabled = true\n\n  site_config {}\n}\n"
        );
    }

    #[test]
    fn tf_resource_app_settings_listed_in_order() {
        let t = tf_resource(&app(false, true, None, &[("A", "1"), ("B", "2")]));
        assert!(t.contains("\n  identity {\n    type = \"SystemAssigned\"\n  }\n"), "{t}");
        assert!(t.ends_with("\n  app_settings = {\n    A = \"1\"\n    B = \"2\"\n  }\n}\n"), "{t}");
    }

    #[test]
    fn tf_resource_sql_server_links_database_to_server_id() {
        let r = Resource::SqlServer {
            name: "sql-x".into(),
            database: "db1".into(),
            sku: "S0".into(),
        };
        assert_eq!(
            tf_resource(&r),
            "resource \"azurerm_mssql_server\" \"sql_x\" {\n  name                          = \"sql-x\"\n  location                      = var.location\n  resource_group_name           = var.resource_group_name\n  version                       = \"12.0\"\n  minimum_tls_version           = \"1.2\"\n  public_network_access_enabled = false\n}\n\nresource \"azurerm_mssql_database\" \"sql_x_db\" {\n  name      = \"db1\"\n  server_id = azurerm_mssql_server.sql_x.id\n  sku_name  = \"S0\"\n}\n"
        );
    }

    #[test]
    fn tf_resource_key_vault_is_private_with_rbac() {
        let r = Resource::KeyVault {
            name: "kv-x".into(),
            private_endpoint_subnet: None,
        };
        assert_eq!(
            tf_resource(&r),
            "resource \"azurerm_key_vault\" \"kv_x\" {\n  name                          = \"kv-x\"\n  location                      = var.location\n  resource_group_name           = var.resource_group_name\n  tenant_id                     = data.azurerm_client_config.current.tenant_id\n  sku_name                      = \"standard\"\n  rbac_authorization_enabled    = true\n  public_network_access_enabled = false\n}\n"
        );
    }

    #[test]
    fn tf_resource_storage_uses_redundancy_as_replication_type() {
        let r = Resource::Storage {
            name: "stx".into(),
            redundancy: "GRS".into(),
        };
        assert_eq!(
            tf_resource(&r),
            "resource \"azurerm_storage_account\" \"stx\" {\n  name                     = \"stx\"\n  location                 = var.location\n  resource_group_name      = var.resource_group_name\n  account_tier             = \"Standard\"\n  account_replication_type = \"GRS\"\n  min_tls_version          = \"TLS1_2\"\n  allow_nested_items_to_be_public = false\n}\n"
        );
    }

    #[test]
    fn generate_empty_design_emits_only_the_header() {
        assert_eq!(
            generate(&design(vec![]), Target::Bicep),
            "// Generated by CADE from the Architecture canvas. Edits here sync back to the canvas.\ntargetScope = 'resourceGroup'\nmetadata resourceGroup = 'rg-test'\n\nparam location string = 'westus3'\n\n"
        );
        assert_eq!(
            generate(&design(vec![]), Target::Terraform),
            "# Generated by CADE from the Architecture canvas. Edits here sync back to the canvas.\nterraform {\n  required_providers {\n    azurerm = {\n      source  = \"hashicorp/azurerm\"\n      version = \"~> 4.0\"\n    }\n  }\n}\n\nprovider \"azurerm\" {\n  features {}\n}\n\nvariable \"location\" {\n  type    = string\n  default = \"westus3\"\n}\n\nvariable \"resource_group_name\" {\n  type    = string\n  default = \"rg-test\"\n}\n\ndata \"azurerm_client_config\" \"current\" {}\n\n"
        );
    }

    #[test]
    fn missing_settings_combines_every_app_and_keeps_read_order() {
        let mut second = app(false, false, None, &[("QUEUE", "q")]);
        if let Resource::AppService { name, .. } = &mut second {
            *name = "app-worker".into();
        }
        let d = design(vec![app(false, false, None, &[("DB", "x")]), second]);
        assert_eq!(missing_settings(&d, &["Z", "DB", "QUEUE", "A"]), vec!["Z", "A"]);
    }

    #[test]
    fn symbol_leading_digit_gets_letter_prefix() {
        let r = Resource::Storage {
            name: "1st-store".into(),
            redundancy: "LRS".into(),
        };
        assert_eq!(r.symbol(), "r1stStore");
    }

    #[test]
    fn label_leading_digit_gets_underscore_prefix() {
        let r = Resource::Storage {
            name: "1st-store".into(),
            redundancy: "LRS".into(),
        };
        assert_eq!(r.label(), "_1st_store");
    }

    #[test]
    fn bicep_resource_hostile_setting_value_is_escaped() {
        let b = bicep_resource(&app(false, false, None, &[("GREETING", "it's ${var.x} \"ok\" \\ end\nnext")]));
        assert!(
            b.contains("        { name: 'GREETING', value: 'it\\'s \\${var.x} \"ok\" \\\\ end\\nnext' }\n"),
            "{b}"
        );
    }

    #[test]
    fn tf_resource_hostile_setting_value_is_escaped() {
        let t = tf_resource(&app(false, false, None, &[("GREETING", "it's ${var.x} %{if} \"ok\" \\ end\nnext")]));
        assert!(
            t.contains("    GREETING = \"it's $${var.x} %%{if} \\\"ok\\\" \\\\ end\\nnext\"\n"),
            "{t}"
        );
    }

    #[test]
    fn tf_resource_non_identifier_setting_key_is_quoted() {
        let t = tf_resource(&app(false, false, None, &[("my.key", "v"), ("x\" = 1 #", "w")]));
        assert!(t.contains("    \"my.key\" = \"v\"\n"), "{t}");
        assert!(t.contains("    \"x\\\" = 1 #\" = \"w\"\n"), "{t}");
    }

    #[test]
    fn bicep_resource_quote_and_backslash_in_name_are_escaped() {
        let r = Resource::Storage {
            name: "st'x\\y".into(),
            redundancy: "LRS".into(),
        };
        let b = bicep_resource(&r);
        assert!(
            b.starts_with("resource stXY 'Microsoft.Storage/storageAccounts@2023-05-01' = {\n  name: 'st\\'x\\\\y'\n"),
            "{b}"
        );
    }

    #[test]
    fn tf_resource_hostile_name_yields_valid_label_and_escaped_name() {
        let r = Resource::Storage {
            name: "st\"x y".into(),
            redundancy: "LRS\" # x".into(),
        };
        let t = tf_resource(&r);
        assert!(
            t.starts_with("resource \"azurerm_storage_account\" \"st_x_y\" {\n  name                     = \"st\\\"x y\"\n"),
            "{t}"
        );
        assert!(t.contains("  account_replication_type = \"LRS\\\" # x\"\n"), "{t}");
    }

    const SUBNET: &str = "/subscriptions/sub/resourceGroups/rg-net/providers/Microsoft.Network/virtualNetworks/vnet/subnets/snet-app";

    #[test]
    fn bicep_resource_app_emits_linux_plan_with_sku_and_references_it() {
        let b = bicep_resource(&app(true, false, None, &[]));
        assert!(
            b.starts_with("resource appWebPlan 'Microsoft.Web/serverfarms@2023-12-01' = {\n  name: 'plan-app-web'\n  location: location\n  kind: 'linux'\n  sku: { name: 'B1' }\n  properties: {\n    reserved: true\n  }\n}\n\n"),
            "{b}"
        );
        assert!(b.contains("  kind: 'app,linux'\n"), "{b}");
        assert!(b.contains("    serverFarmId: appWebPlan.id\n"), "{b}");
    }

    #[test]
    fn tf_resource_app_emits_linux_service_plan_and_references_it() {
        let t = tf_resource(&app(true, false, None, &[]));
        assert!(
            t.starts_with("resource \"azurerm_service_plan\" \"app_web\" {\n  name                = \"plan-app-web\"\n  location            = var.location\n  resource_group_name = var.resource_group_name\n  os_type             = \"Linux\"\n  sku_name            = \"B1\"\n}\n\n"),
            "{t}"
        );
        assert!(t.contains("  service_plan_id     = azurerm_service_plan.app_web.id\n"), "{t}");
    }

    #[test]
    fn bicep_resource_app_with_subnet_integrates_with_quoted_subnet_id() {
        let b = bicep_resource(&app(false, false, Some(SUBNET), &[]));
        assert!(b.contains(&format!("    virtualNetworkSubnetId: '{SUBNET}'\n")), "{b}");
    }

    #[test]
    fn tf_resource_app_with_subnet_sets_vnet_integration() {
        let t = tf_resource(&app(false, false, Some(SUBNET), &[]));
        assert!(t.contains(&format!("  virtual_network_subnet_id = \"{SUBNET}\"\n")), "{t}");
    }

    #[test]
    fn bicep_resource_key_vault_with_subnet_emits_private_endpoint() {
        let r = Resource::KeyVault {
            name: "kv-x".into(),
            private_endpoint_subnet: Some(SUBNET.into()),
        };
        let b = bicep_resource(&r);
        assert!(
            b.contains(&format!(
                "}}\n\nresource kvXPe 'Microsoft.Network/privateEndpoints@2023-11-01' = {{\n  name: 'pe-kv-x'\n  location: location\n  properties: {{\n    subnet: {{ id: '{SUBNET}' }}\n    privateLinkServiceConnections: [\n      {{\n        name: 'kv-x'\n        properties: {{\n          privateLinkServiceId: kvX.id\n          groupIds: [ 'vault' ]\n        }}\n      }}\n    ]\n  }}\n}}\n"
            )),
            "{b}"
        );
    }

    #[test]
    fn tf_resource_key_vault_with_subnet_emits_private_endpoint() {
        let r = Resource::KeyVault {
            name: "kv-x".into(),
            private_endpoint_subnet: Some(SUBNET.into()),
        };
        let t = tf_resource(&r);
        assert!(
            t.ends_with(&format!(
                "}}\n\nresource \"azurerm_private_endpoint\" \"kv_x\" {{\n  name                = \"pe-kv-x\"\n  location            = var.location\n  resource_group_name = var.resource_group_name\n  subnet_id           = \"{SUBNET}\"\n\n  private_service_connection {{\n    name                           = \"kv-x\"\n    private_connection_resource_id = azurerm_key_vault.kv_x.id\n    subresource_names              = [\"vault\"]\n    is_manual_connection           = false\n  }}\n\n  private_dns_zone_group {{\n    name                 = \"default\"\n    private_dns_zone_ids = [azurerm_private_dns_zone.vault.id]\n  }}\n}}\n"
            )),
            "{t}"
        );
    }

    #[test]
    fn generate_bicep_uses_design_location_and_resource_group() {
        let b = generate(&design(vec![]), Target::Bicep);
        assert!(
            b.contains("\ntargetScope = 'resourceGroup'\nmetadata resourceGroup = 'rg-test'\n\nparam location string = 'westus3'\n"),
            "{b}"
        );
    }

    #[test]
    fn generate_terraform_uses_design_location_and_resource_group() {
        let t = generate(&design(vec![]), Target::Terraform);
        assert!(
            t.contains("variable \"location\" {\n  type    = string\n  default = \"westus3\"\n}\n"),
            "{t}"
        );
        assert!(
            t.contains("variable \"resource_group_name\" {\n  type    = string\n  default = \"rg-test\"\n}\n"),
            "{t}"
        );
        assert!(t.contains("provider \"azurerm\" {\n  features {}\n}\n"), "{t}");
    }

    const VNET_A: &str = "/subscriptions/sub/resourceGroups/rg-net/providers/Microsoft.Network/virtualNetworks/vnet-a";
    const VNET_B: &str = "/subscriptions/sub/resourceGroups/rg-hub/providers/Microsoft.Network/virtualNetworks/vnet-b";

    fn vault(name: &str, subnet: Option<String>) -> Resource {
        Resource::KeyVault {
            name: name.into(),
            private_endpoint_subnet: subnet,
        }
    }

    #[test]
    fn vnet_id_subnet_resource_id_returns_its_virtual_network() {
        assert_eq!(vnet_id(&format!("{VNET_A}/subnets/snet-data")), Some(VNET_A));
        assert_eq!(vnet_id(&format!("{VNET_A}/SUBNETS/snet-data")), Some(VNET_A));
    }

    #[test]
    fn vnet_id_value_without_subnets_segment_returns_none() {
        assert_eq!(vnet_id(VNET_A), None);
        assert_eq!(vnet_id("snet-data"), None);
        assert_eq!(vnet_id("/subnets/snet-data"), None);
    }

    #[test]
    fn bicep_resource_key_vault_with_subnet_emits_dns_zone_group() {
        let b = bicep_resource(&vault("kv-x", Some(SUBNET.into())));
        assert!(
            b.ends_with(
                "}\n\nresource kvXPeDns 'Microsoft.Network/privateEndpoints/privateDnsZoneGroups@2023-11-01' = {\n  parent: kvXPe\n  name: 'default'\n  properties: {\n    privateDnsZoneConfigs: [\n      {\n        name: 'privatelink-vaultcore-azure-net'\n        properties: {\n          privateDnsZoneId: vaultDnsZone.id\n        }\n      }\n    ]\n  }\n}\n"
            ),
            "{b}"
        );
    }

    #[test]
    fn tf_resource_key_vault_with_subnet_emits_dns_zone_group() {
        let t = tf_resource(&vault("kv-x", Some(SUBNET.into())));
        assert!(
            t.contains(
                "    is_manual_connection           = false\n  }\n\n  private_dns_zone_group {\n    name                 = \"default\"\n    private_dns_zone_ids = [azurerm_private_dns_zone.vault.id]\n  }\n}\n"
            ),
            "{t}"
        );
    }

    fn two_vnets() -> Design {
        design(vec![
            vault("kv-a", Some(format!("{VNET_A}/subnets/snet-1"))),
            vault("kv-b", Some(format!("{VNET_A}/subnets/snet-2"))),
            vault("kv-c", Some(format!("{VNET_B}/subnets/snet-1"))),
            vault("kv-public", None),
        ])
    }

    #[test]
    fn vault_dns_bicep_emits_one_zone_and_one_link_per_virtual_network() {
        assert_eq!(
            vault_dns(&two_vnets(), Target::Bicep),
            format!(
                "resource vaultDnsZone 'Microsoft.Network/privateDnsZones@2024-06-01' = {{\n  name: 'privatelink.vaultcore.azure.net'\n  location: 'global'\n}}\n\nresource vaultDnsLink1 'Microsoft.Network/privateDnsZones/virtualNetworkLinks@2024-06-01' = {{\n  parent: vaultDnsZone\n  name: 'link-vnet-a'\n  location: 'global'\n  properties: {{\n    registrationEnabled: false\n    virtualNetwork: {{ id: '{VNET_A}' }}\n  }}\n}}\n\nresource vaultDnsLink2 'Microsoft.Network/privateDnsZones/virtualNetworkLinks@2024-06-01' = {{\n  parent: vaultDnsZone\n  name: 'link-vnet-b'\n  location: 'global'\n  properties: {{\n    registrationEnabled: false\n    virtualNetwork: {{ id: '{VNET_B}' }}\n  }}\n}}\n\n"
            )
        );
    }

    #[test]
    fn vault_dns_terraform_emits_one_zone_and_one_link_per_virtual_network() {
        assert_eq!(
            vault_dns(&two_vnets(), Target::Terraform),
            format!(
                "resource \"azurerm_private_dns_zone\" \"vault\" {{\n  name                = \"privatelink.vaultcore.azure.net\"\n  resource_group_name = var.resource_group_name\n}}\n\nresource \"azurerm_private_dns_zone_virtual_network_link\" \"vault_1\" {{\n  name                  = \"link-vnet-a\"\n  resource_group_name   = var.resource_group_name\n  private_dns_zone_name = azurerm_private_dns_zone.vault.name\n  virtual_network_id    = \"{VNET_A}\"\n  registration_enabled  = false\n}}\n\nresource \"azurerm_private_dns_zone_virtual_network_link\" \"vault_2\" {{\n  name                  = \"link-vnet-b\"\n  resource_group_name   = var.resource_group_name\n  private_dns_zone_name = azurerm_private_dns_zone.vault.name\n  virtual_network_id    = \"{VNET_B}\"\n  registration_enabled  = false\n}}\n\n"
            )
        );
    }

    #[test]
    fn vault_dns_same_vnet_name_in_two_groups_gets_distinct_link_names() {
        let other = VNET_B.replace("vnet-b", "vnet-a");
        let d = design(vec![
            vault("kv-a", Some(format!("{VNET_A}/subnets/s"))),
            vault("kv-b", Some(format!("{other}/subnets/s"))),
        ]);
        let b = vault_dns(&d, Target::Bicep);
        assert!(b.contains("  name: 'link-vnet-a'\n"), "{b}");
        assert!(b.contains("  name: 'link-vnet-a-2'\n"), "{b}");
    }

    #[test]
    fn vault_dns_endpoint_subnet_without_vnet_emits_zone_but_no_link() {
        let d = design(vec![vault("kv-a", Some("snet-data".into()))]);
        let t = vault_dns(&d, Target::Terraform);
        assert!(t.contains("resource \"azurerm_private_dns_zone\" \"vault\""), "{t}");
        assert!(!t.contains("virtual_network_link"), "{t}");
    }

    #[test]
    fn vault_dns_no_private_endpoint_emits_nothing() {
        let d = design(vec![vault("kv-public", None), app(true, false, Some(SUBNET), &[])]);
        assert_eq!(vault_dns(&d, Target::Bicep), "");
        assert_eq!(vault_dns(&d, Target::Terraform), "");
    }

    #[test]
    fn generate_key_vault_endpoint_includes_vault_dns() {
        let d = design(vec![vault("kv-a", Some(format!("{VNET_A}/subnets/s")))]);
        for target in [Target::Bicep, Target::Terraform] {
            let g = generate(&d, target);
            assert!(g.ends_with(&vault_dns(&d, target)), "{g}");
            assert!(g.contains("privatelink.vaultcore.azure.net"), "{g}");
        }
    }

    #[test]
    fn missing_settings_no_app_services_reports_every_read() {
        let d = design(vec![Resource::Storage {
            name: "st".into(),
            redundancy: "LRS".into(),
        }]);
        assert_eq!(missing_settings(&d, &["DB"]), vec!["DB"]);
        assert!(missing_settings(&claims(), &[]).is_empty());
    }
}
