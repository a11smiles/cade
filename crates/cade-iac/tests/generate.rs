//! Whole-design generation compared with checked-in golden files, and the
//! JSON shape of a design (it is stored in the Architecture project).

use cade_iac::{Design, Resource, Target, generate};

const NET: &str = "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg-claims-net/providers/Microsoft.Network/virtualNetworks/vnet-claims/subnets";

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
                subnet: Some(format!("{NET}/snet-app")),
                settings: vec![("DB_NAME".into(), "claims".into())],
            },
            Resource::SqlServer {
                name: "sql-claims".into(),
                database: "claims".into(),
                sku: "GP_S_Gen5_2".into(),
            },
            Resource::KeyVault {
                name: "kv-claims".into(),
                private_endpoint_subnet: Some(format!("{NET}/snet-data")),
            },
            Resource::Storage {
                name: "stclaimsphotos".into(),
                redundancy: "ZRS".into(),
            },
        ],
    }
}

#[test]
fn generate_claims_design_bicep_matches_golden_file() {
    assert_eq!(generate(&claims(), Target::Bicep), include_str!("golden/claims.bicep"));
}

#[test]
fn generate_claims_design_terraform_matches_golden_file() {
    assert_eq!(generate(&claims(), Target::Terraform), include_str!("golden/claims.tf"));
}

#[test]
fn design_serde_resources_tagged_with_arm_types_round_trip() {
    let d = claims();
    let json = serde_json::to_value(&d).unwrap();
    let types: Vec<&str> = json["resources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["type"].as_str().unwrap())
        .collect();
    assert_eq!(
        types,
        [
            "Microsoft.Web/sites",
            "Microsoft.Sql/servers",
            "Microsoft.KeyVault/vaults",
            "Microsoft.Storage/storageAccounts"
        ]
    );
    assert_eq!(json["resources"][0]["settings"], serde_json::json!([["DB_NAME", "claims"]]));
    assert_eq!(serde_json::from_value::<Design>(json).unwrap(), d);
}
