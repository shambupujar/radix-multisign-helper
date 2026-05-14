use serde::{Deserialize, Serialize};

use crate::utils::gateway_base_url;

#[derive(Serialize, Deserialize)]
struct GatewayStatus {
    ledger_state: LedgerState,
}

#[derive(Serialize, Deserialize)]
struct LedgerState {
    epoch: u64,
}

pub async fn get_epoch(network_id: u8) -> reqwest::Result<u64> {
    let base_url = gateway_base_url(network_id);
    Ok(reqwest::Client::new()
        .post(format!("{}/status/gateway-status", base_url))
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 Edg/118.0.2088.46")
        .send()
        .await
        .unwrap().json::<GatewayStatus>()
        .await.unwrap().ledger_state.epoch)
        // .and_then(|response| Ok(response.json::<GatewayStatus>()))
        // .map(|response| response.ledger_state.epoch)
}
