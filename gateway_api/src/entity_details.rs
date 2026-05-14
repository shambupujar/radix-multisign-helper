use serde::{Deserialize, Serialize};
use crate::utils::gateway_base_url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {}

pub async fn get_validator_details(validator_address: &String, network_id: u8) -> reqwest::Result<serde_json::Value> {
    let base_url = gateway_base_url(network_id);
    let response : serde_json:: Value = reqwest::Client::new()
        .post(format!("{}/state/entity/details", base_url))
        .json(&serde_json::json!({
            "addresses": [
              validator_address
            ],
            "aggregation_level": "Vault",
            "opt_ins": {
              "ancestor_identities": true,
              "component_royalty_vault_balance": true,
              "package_royalty_vault_balance": true,
              "non_fungible_include_nfids": true,
              "explicit_metadata": [
                "name",
                "description"
              ]
            }
          }))
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 Edg/118.0.2088.46")
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response)
}
