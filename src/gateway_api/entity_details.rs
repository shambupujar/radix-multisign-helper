use serde::{Deserialize, Serialize};

struct Entity {}

pub fn get_validator_details(validator_address: &String) -> reqwest::Result<T> {
    let response : serde_json:: Value = reqwest::blocking::Client::new()
        .post(format!("{}/state/entity/details", constants::GATEWAY_API_BASE_URL))
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
}
