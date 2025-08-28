use std::{thread, time};
use serde::{Deserialize, Serialize};
use crate::utils::GATEWAY_API_BASE_URL;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionSubmitRequest {
    pub notarized_transaction_hex: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionStatusRequest {
    pub intent_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionStatusResponse {
    pub status: String,
}

pub async fn submit_gateway_txn(
    bech32m_intent_hash: &str,
    notarized_txn_hex: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    // Submit transaction
    let submit_request = TransactionSubmitRequest {
        notarized_transaction_hex: notarized_txn_hex.to_owned(),
    };
    
    let _submit_response = client
        .post(format!("{}/transaction/submit", GATEWAY_API_BASE_URL))
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 Edg/118.0.2088.46")
        .json(&submit_request)
        .send()
        .await?;

    // Check status
    let status_request = TransactionStatusRequest {
        intent_hash: bech32m_intent_hash.to_owned(),
    };
    
    let mut n = 1;
    let mut status = "Pending".to_string();
    
    while status == "Pending" && n < 30 {
        let status_response = client
            .post(format!("{}/transaction/status", GATEWAY_API_BASE_URL))
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 Edg/118.0.2088.46")
            .json(&status_request)
            .send()
            .await?;
            
        if let Ok(response) = status_response.json::<TransactionStatusResponse>().await {
            status = response.status;
        }
        
        thread::sleep(time::Duration::from_secs(1));
        n += 1;
    }

    if status != "CommittedSuccess" {
        println!("Transaction Submission failed {}", bech32m_intent_hash);
    } else {
        println!("Transaction Successful {}", bech32m_intent_hash);
    }
    
    Ok(())
}
