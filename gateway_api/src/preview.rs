use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::utils::gateway_base_url;

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviewRequest {
    pub manifest: String,
    pub start_epoch_inclusive: u64,
    pub end_epoch_exclusive: u64,
    pub tip_percentage: u16,
    pub nonce: u64,
    pub signer_public_keys: Vec<HashMap<String, String>>,
    pub flags: Flags,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flags {
    pub use_free_credit: bool,
    pub assume_all_signature_proofs: bool,
    pub skip_epoch_check: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviewResponse {
    pub message: Option<Message>,
    pub receipt: Option<Receipt>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub receipt: Receipt,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Receipt {
    pub status: String,
}
pub async fn txn_preview(preview_request: PreviewRequest, network_id: u8) -> Result<PreviewResponse, Box<dyn std::error::Error>> {
    let base_url = gateway_base_url(network_id);
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/transaction/preview", base_url))
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 Edg/118.0.2088.46")
        .json(&preview_request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_text = response.text().await?;
            println!("Response text: {}", response_text);
            let preview_response: PreviewResponse = serde_json::from_str(&response_text)?;
            Ok(preview_response)
        }
        _ => {
            let error = response.text().await?;
            Err(error.into())
        }
    }

}
