mod commands;

use anyhow::Result;
use commands::{prepare_intent_hash, sign_intent_hash, submit_transaction};
use inquire::Select;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔐 Multisign Helper - Interactive Transaction Signing Tool");
    println!();

    let commands = vec![
        "prepare-intent-hash",
        "sign-intent-hash", 
        "submit-trxn",
        "exit"
    ];

    loop {
        let command = Select::new("Select a command:", commands.clone())
            .prompt()
            .map_err(|e| anyhow::anyhow!("Selection error: {}", e))?;

        match command {
            "prepare-intent-hash" => {
                if let Err(e) = prepare_intent_hash().await {
                    eprintln!("Error preparing intent hash: {}", e);
                }
            },
            "sign-intent-hash" => {
                if let Err(e) = sign_intent_hash() {
                    eprintln!("Error signing intent hash: {}", e);
                }
            },
            "submit-trxn" => {
                if let Err(e) = submit_transaction().await {
                    eprintln!("Error submitting transaction: {}", e);
                }
            },
            "exit" => {
                println!("Goodbye! 👋");
                break;
            },
            _ => {
                println!("Unknown command: {}", command);
            }
        }
        
        println!();
    }

    Ok(())
}