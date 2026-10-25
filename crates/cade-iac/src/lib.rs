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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Resource {
    #[serde(rename = "Microsoft.Web/sites")]
    AppService {
        name: String,
        plan_sku: String,
        public_access: bool,
        system_identity: bool,
        subnet: Option<String>,
        settings: Vec<(String, String)>,
    },
    #[serde(rename = "Microsoft.Sql/servers")]
    SqlServer { name: String, database: String, sku: String },
    #[serde(rename = "Microsoft.KeyVault/vaults")]
    KeyVault {
        name: String,
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

    /// Bicep symbol name derived from the resource name.
    fn symbol(&self) -> String {
        let mut out = String::new();
        let mut upper = false;
        for (i, ch) in self.name().chars().enumerate() {
            if ch == '-' || ch == '_' {
                upper = true;
            } else if i == 0 {
                out.push(ch.to_ascii_lowercase());
            } else if upper {
                out.push(ch.to_ascii_uppercase());
                upper = false;
            } else {
                out.push(ch);
            }
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Design {
    pub location: String,
    pub resource_group: String,
    pub resources: Vec<Resource>,
}

fn bicep_resource(r: &Resource) -> String {
    let sym = r.symbol();
    match r {
        Resource::AppService {
            name,
            public_access,
            system_identity,
            subnet,
            settings,
            ..
        } => {
            let mut s = format!("resource {sym} 'Microsoft.Web/sites@2023-12-01' = {{\n  name: '{name}'\n  location: location\n");
            if *system_identity {
                s.push_str("  identity: { type: 'SystemAssigned' }\n");
            }
            s.push_str("  properties: {\n    serverFarmId: plan.id\n    httpsOnly: true\n");
            s.push_str(&format!(
                "    publicNetworkAccess: '{}'\n",
                if *public_access { "Enabled" } else { "Disabled" }
            ));
            if let Some(sn) = subnet {
                s.push_str(&format!("    virtualNetworkSubnetId: {sn}\n"));
            }
            if !settings.is_empty() {
                s.push_str("    siteConfig: {\n      appSettings: [\n");
                for (k, v) in settings {
                    s.push_str(&format!("        {{ name: '{k}', value: '{v}' }}\n"));
                }
                s.push_str("      ]\n    }\n");
            }
            s.push_str("  }\n}\n");
            s
        }
        Resource::SqlServer { name, database, sku } => format!(
            "resource {sym} 'Microsoft.Sql/servers@2023-08-01' = {{\n  name: '{name}'\n  location: location\n  properties: {{\n    publicNetworkAccess: 'Disabled'\n    minimalTlsVersion: '1.2'\n  }}\n}}\n\nresource {sym}Db 'Microsoft.Sql/servers/databases@2023-08-01' = {{\n  parent: {sym}\n  name: '{database}'\n  location: location\n  sku: {{ name: '{sku}' }}\n}}\n"
        ),
        Resource::KeyVault { name, .. } => format!(
            "resource {sym} 'Microsoft.KeyVault/vaults@2023-07-01' = {{\n  name: '{name}'\n  location: location\n  properties: {{\n    tenantId: subscription().tenantId\n    sku: {{ family: 'A', name: 'standard' }}\n    enableRbacAuthorization: true\n    publicNetworkAccess: 'Disabled'\n  }}\n}}\n"
        ),
        Resource::Storage { name, redundancy } => format!(
            "resource {sym} 'Microsoft.Storage/storageAccounts@2023-05-01' = {{\n  name: '{name}'\n  location: location\n  kind: 'StorageV2'\n  sku: {{ name: 'Standard_{redundancy}' }}\n  properties: {{\n    allowBlobPublicAccess: false\n    minimumTlsVersion: 'TLS1_2'\n  }}\n}}\n"
        ),
    }
}

fn tf_resource(r: &Resource) -> String {
    let id = r.name().replace('-', "_");
    match r {
        Resource::AppService {
            name,
            public_access,
            system_identity,
            settings,
            ..
        } => {
            let mut s = format!(
                "resource \"azurerm_linux_web_app\" \"{id}\" {{\n  name                = \"{name}\"\n  location            = var.location\n  resource_group_name = var.resource_group_name\n  service_plan_id     = azurerm_service_plan.plan.id\n  https_only          = true\n  public_network_access_enabled = {public_access}\n"
            );
            if *system_identity {
                s.push_str("\n  identity {\n    type = \"SystemAssigned\"\n  }\n");
            }
            s.push_str("\n  site_config {}\n");
            if !settings.is_empty() {
                s.push_str("\n  app_settings = {\n");
                for (k, v) in settings {
                    s.push_str(&format!("    {k} = \"{v}\"\n"));
                }
                s.push_str("  }\n");
            }
            s.push_str("}\n");
            s
        }
        Resource::SqlServer { name, database, sku } => format!(
            "resource \"azurerm_mssql_server\" \"{id}\" {{\n  name                          = \"{name}\"\n  location                      = var.location\n  resource_group_name           = var.resource_group_name\n  version                       = \"12.0\"\n  minimum_tls_version           = \"1.2\"\n  public_network_access_enabled = false\n}}\n\nresource \"azurerm_mssql_database\" \"{id}_db\" {{\n  name      = \"{database}\"\n  server_id = azurerm_mssql_server.{id}.id\n  sku_name  = \"{sku}\"\n}}\n"
        ),
        Resource::KeyVault { name, .. } => format!(
            "resource \"azurerm_key_vault\" \"{id}\" {{\n  name                          = \"{name}\"\n  location                      = var.location\n  resource_group_name           = var.resource_group_name\n  tenant_id                     = data.azurerm_client_config.current.tenant_id\n  sku_name                      = \"standard\"\n  rbac_authorization_enabled    = true\n  public_network_access_enabled = false\n}}\n"
        ),
        Resource::Storage { name, redundancy } => format!(
            "resource \"azurerm_storage_account\" \"{id}\" {{\n  name                     = \"{name}\"\n  location                 = var.location\n  resource_group_name      = var.resource_group_name\n  account_tier             = \"Standard\"\n  account_replication_type = \"{redundancy}\"\n  min_tls_version          = \"TLS1_2\"\n  allow_nested_items_to_be_public = false\n}}\n"
        ),
    }
}

pub fn generate(design: &Design, target: Target) -> String {
    match target {
        Target::Bicep => {
            let mut s = String::from(
                "// Generated by CADE from the Architecture canvas. Edits here sync back to the canvas.\nparam location string = resourceGroup().location\n\n",
            );
            for r in &design.resources {
                s.push_str(&bicep_resource(r));
                s.push('\n');
            }
            s
        }
        Target::Terraform => {
            let mut s = String::from(
                "# Generated by CADE from the Architecture canvas. Edits here sync back to the canvas.\nvariable \"location\" { type = string }\nvariable \"resource_group_name\" { type = string }\n\ndata \"azurerm_client_config\" \"current\" {}\n\n",
            );
            for r in &design.resources {
                s.push_str(&tf_resource(r));
                s.push('\n');
            }
            s
        }
    }
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
                    subnet: Some("vnet::appSubnet.id".into()),
                    settings: vec![("DB_NAME".into(), "claims".into())],
                },
                Resource::SqlServer {
                    name: "sql-claims".into(),
                    database: "claims".into(),
                    sku: "GP_S_Gen5_2".into(),
                },
                Resource::KeyVault {
                    name: "kv-claims".into(),
                    private_endpoint_subnet: Some("snet-data".into()),
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
}
