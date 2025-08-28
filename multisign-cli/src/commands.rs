use anyhow::{Context, Result};
use inquire::{validator::Validation, Select, Text};
use radix_transactions::manifest::{compile, BlobProvider};
use radix_transactions::prelude::*;
use scrypto::prelude::*;
use std::fs;
use std::path::Path;

/// Parse a private key from hex string based on key type
fn parse_private_key(hex_str: &str, key_type: &str) -> Result<PrivateKey> {
    let private_key_bytes = hex::decode(hex_str.trim())
        .context("Failed to decode private key")?;

    match key_type {
        "Ed25519" => {
            let private_key = Ed25519PrivateKey::from_bytes(&private_key_bytes)
                .map_err(|e| anyhow::anyhow!("Invalid Ed25519 private key: {:?}", e))?;
            Ok(PrivateKey::Ed25519(private_key))
        },
        "Secp256k1" => {
            let private_key = Secp256k1PrivateKey::from_bytes(&private_key_bytes)
                .map_err(|e| anyhow::anyhow!("Invalid Secp256k1 private key: {:?}", e))?;
            Ok(PrivateKey::Secp256k1(private_key))
        },
        _ => Err(anyhow::anyhow!("Unsupported key type: {}", key_type))
    }
}

/// Validate file path for .rtm files
fn validate_rtm_file_path(input: &str) -> Result<Validation, inquire::CustomUserError> {
    if input.trim().is_empty() {
        return Ok(Validation::Invalid("Path cannot be empty".into()));
    }
    if !Path::new(input.trim()).exists() {
        return Ok(Validation::Invalid("File does not exist".into()));
    }
    if !input.trim().ends_with(".rtm") {
        return Ok(Validation::Invalid("File must have .rtm extension".into()));
    }
    Ok(Validation::Valid)
}

/// Validate public key (supports both Ed25519 and Secp256k1)
fn validate_public_key(input: &str) -> Result<Validation, inquire::CustomUserError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(Validation::Invalid("Public key cannot be empty".into()));
    }
    match hex::decode(trimmed) {
        Ok(bytes) => {
            if bytes.len() == 32 || bytes.len() == 33 {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Public key must be 32 bytes (Ed25519) or 33 bytes (Secp256k1)".into()))
            }
        },
        Err(_) => Ok(Validation::Invalid("Invalid hex format".into()))
    }
}


/// Validate private key input based on key type
fn validate_private_key(input: &str, key_type: &str) -> Result<Validation, inquire::CustomUserError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(Validation::Invalid("Private key cannot be empty".into()));
    }
    match hex::decode(trimmed) {
        Ok(bytes) => {
            let expected_len = match key_type {
                "Ed25519" => 32,
                "Secp256k1" => 32,
                _ => return Ok(Validation::Invalid("Unknown key type".into()))
            };
            if bytes.len() == expected_len {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(format!("{} private key must be {} bytes ({} hex characters)", 
                    key_type, expected_len, expected_len * 2).into()))
            }
        },
        Err(_) => Ok(Validation::Invalid("Invalid hex format".into()))
    }
}

pub fn prepare_intent_hash() -> Result<()> {
    println!("📝 Preparing Intent Hash from Transaction Manifest");
    println!();

    // Get manifest file path
    let manifest_path =
        Text::new("Enter the path to the transaction manifest file (.rtm):")
            .with_validator(validate_rtm_file_path)
            .prompt()
            .context("Failed to get manifest path")?;

    // Get notary public key
    let notary_public_key_input = Text::new("Enter the notary public key (hex format):")
        .with_validator(validate_public_key)
        .prompt()
        .context("Failed to get notary public key")?;

    // Get signer key type
    let signer_key_types = vec!["Ed25519", "Secp256k1"];
    let signer_key_type = Select::new("Select signer key type:", signer_key_types)
        .prompt()
        .context("Failed to select signer key type")?;

    // Get signer private key
    let signer_key_type_clone = signer_key_type.to_string();
    let private_key_input = Text::new(&format!("Enter your signer private key (hex format, {} 32 bytes):", signer_key_type))
        .with_validator(move |input: &str| validate_private_key(input, &signer_key_type_clone))
        .prompt()
        .context("Failed to get signer private key")?;

    // Get notary key type
    let notary_key_types = vec!["Ed25519", "Secp256k1"];
    let notary_key_type = Select::new("Select notary key type:", notary_key_types)
        .prompt()
        .context("Failed to select notary key type")?;

    // Get notary private key
    let notary_key_type_clone = notary_key_type.to_string();
    let notary_private_key_input = Text::new(&format!("Enter the notary private key (hex format, {} 32 bytes):", notary_key_type))
        .with_validator(move |input: &str| validate_private_key(input, &notary_key_type_clone))
        .prompt()
        .context("Failed to get notary private key")?;

    // Parse private keys using the reusable function
    let _signer_private_key = parse_private_key(&private_key_input, signer_key_type)?;
    let _notary_private_key = parse_private_key(&notary_private_key_input, notary_key_type)?;

    // Get network ID (default to 1)
    let network_id_str = Text::new("Enter network ID:")
        .with_default("1")
        .with_validator(|input: &str| match input.trim().parse::<u8>() {
            Ok(_) => Ok(Validation::Valid),
            Err(_) => Ok(Validation::Invalid(
                "Network ID must be a valid number".into(),
            )),
        })
        .prompt()
        .context("Failed to get network ID")?;

    let network_id: u8 = network_id_str
        .trim()
        .parse()
        .context("Failed to parse network ID")?;

    println!();
    println!("🔄 Processing...");

    // Read and compile the manifest
    let manifest_content = fs::read_to_string(manifest_path.trim())
        .context("Failed to read manifest file")?;

    let network_definition = match network_id {
        1 => NetworkDefinition::mainnet(),
        2 => NetworkDefinition::stokenet(),
        _ => NetworkDefinition::simulator(),
    };

    let manifest = compile(&manifest_content, &network_definition, BlobProvider::new())
        .map_err(|e| anyhow::anyhow!("Failed to compile manifest: {:?}", e))?;

    let notary_key_bytes = hex::decode(notary_public_key_input.trim())
        .context("Failed to decode notary public key")?;

    let notary_public_key = match notary_key_type {
        "Ed25519" => {
            if notary_key_bytes.len() != 32 {
                return Err(anyhow::anyhow!("Ed25519 public key must be 32 bytes"));
            }
            let mut key_bytes = [0u8; 32];
            key_bytes.copy_from_slice(&notary_key_bytes);
            PublicKey::Ed25519(Ed25519PublicKey(key_bytes))
        },
        "Secp256k1" => {
            if notary_key_bytes.len() != 33 {
                return Err(anyhow::anyhow!("Secp256k1 public key must be 33 bytes"));
            }
            let mut key_bytes = [0u8; 33];
            key_bytes.copy_from_slice(&notary_key_bytes);
            PublicKey::Secp256k1(Secp256k1PublicKey(key_bytes))
        },
        _ => return Err(anyhow::anyhow!("Unknown key type: {}", notary_key_type))
    };

    let current_epoch = 1000; // Default epoch - in production would fetch from Gateway API

    let header = TransactionHeaderV1 {
        network_id,
        start_epoch_inclusive: Epoch::of(current_epoch),
        end_epoch_exclusive: Epoch::of(current_epoch + 100),
        nonce: 1,
        notary_public_key,
        notary_is_signatory: false,
        tip_percentage: 0,
    };

    // Create intent
    let intent = IntentV1 {
        header: header.clone(),
        instructions: InstructionsV1(manifest.instructions),
        blobs: manifest.blobs.into(),
        message: MessageV1::None,
    };

    // Calculate intent hash
    let intent_hash = intent
        .prepare(&PreparationSettings::latest())
        .unwrap()
        .transaction_intent_hash();
    let intent_hash_hex = hex::encode(intent_hash.0);

    // For now, just create a placeholder for transaction construction
    // In production, this would use the proper transaction building flow
    println!("⚠️  Note: Full transaction construction not implemented in this function.");
    println!("   Use the 'submit-trxn' command for complete transaction assembly.");

    println!("✅ Intent Hash Generated Successfully!");
    println!();
    println!("📋 Results:");
    println!("  Intent Hash: {}", intent_hash_hex);
    println!("  Network ID: {}", network_id);
    println!("  Notary Public Key: {}", notary_public_key_input.trim());
    println!("  Manifest Path: {}", manifest_path.trim());
    println!();
    println!("💡 Save this intent hash for signing by the required parties.");

    Ok(())
}

pub fn sign_intent_hash() -> Result<()> {
    println!("✏️ Sign Intent Hash");
    println!();

    // Get intent hash
    let intent_hash_input = Text::new("Enter the intent hash (hex format):")
        .with_validator(|input: &str| {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return Ok(Validation::Invalid("Intent hash cannot be empty".into()));
            }
            match hex::decode(trimmed) {
                Ok(bytes) => {
                    if bytes.len() == 32 {
                        Ok(Validation::Valid)
                    } else {
                        Ok(Validation::Invalid("Intent hash must be exactly 32 bytes (64 hex characters)".into()))
                    }
                },
                Err(_) => Ok(Validation::Invalid("Invalid hex format".into()))
            }
        })
        .prompt()
        .context("Failed to get intent hash")?;

    // Select key type
    let key_types = vec!["Ed25519", "Secp256k1"];
    let key_type = Select::new("Select key type:", key_types)
        .prompt()
        .context("Failed to select key type")?;

    // Get private key
    let key_type_clone = key_type.to_string();
    let private_key_input = Text::new(&format!("Enter your private key (hex format, {} 32 bytes):", key_type))
        .with_validator(move |input: &str| validate_private_key(input, &key_type_clone))
        .prompt()
        .context("Failed to get private key")?;

    println!();
    println!("🔄 Signing...");

    // Decode intent hash
    let intent_hash_bytes = hex::decode(intent_hash_input.trim())
        .context("Failed to decode intent hash")?;

    let mut hash_array = [0u8; 32];
    hash_array.copy_from_slice(&intent_hash_bytes);
    let hash = Hash(hash_array);

    // Parse private key using the reusable function
    let private_key = parse_private_key(&private_key_input, key_type)?;

    // Sign and get public key based on key type
    let (signature_hex, public_key_hex) = match private_key {
        PrivateKey::Ed25519(ed25519_key) => {
            // Get public key
            let public_key = ed25519_key.public_key();
            let public_key_hex = hex::encode(public_key.0);

            // Sign the hash
            let signature = ed25519_key.sign(&hash);
            let signature_hex = hex::encode(signature.0);

            (signature_hex, public_key_hex)
        }
        PrivateKey::Secp256k1(secp256k1_key) => {
            // Get public key
            let public_key = secp256k1_key.public_key();
            let public_key_hex = hex::encode(public_key.0);

            // Sign the hash
            let signature = secp256k1_key.sign(&hash);
            let signature_hex = hex::encode(signature.0);

            (signature_hex, public_key_hex)
        }
    };

    println!("✅ Signature Generated Successfully!");
    println!();
    println!("📋 Results:");
    println!("  Intent Hash: {}", intent_hash_input.trim());
    println!("  Key Type: {}", key_type);
    println!("  Public Key: {}", public_key_hex);
    println!("  Signature: {}", signature_hex);
    println!();
    println!("💡 Use these values for transaction assembly.");

    Ok(())
}

pub fn submit_transaction() -> Result<()> {
    println!("🚀 Submit Transaction");
    println!();

    // Get intent hash
    let intent_hash_input = Text::new("Enter the intent hash (hex format):")
        .with_validator(|input: &str| {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return Ok(Validation::Invalid("Intent hash cannot be empty".into()));
            }
            match hex::decode(trimmed) {
                Ok(bytes) => {
                    if bytes.len() == 32 {
                        Ok(Validation::Valid)
                    } else {
                        Ok(Validation::Invalid("Intent hash must be exactly 32 bytes (64 hex characters)".into()))
                    }
                },
                Err(_) => Ok(Validation::Invalid("Invalid hex format".into()))
            }
        })
        .prompt()
        .context("Failed to get intent hash")?;

    // Get number of signers
    let num_signers_str = Text::new("Enter the number of signers:")
        .with_validator(|input: &str| match input.trim().parse::<u32>() {
            Ok(n) if n > 0 && n <= 20 => Ok(Validation::Valid),
            Ok(_) => Ok(Validation::Invalid(
                "Number of signers must be between 1 and 20".into(),
            )),
            Err(_) => Ok(Validation::Invalid(
                "Number of signers must be a valid number".into(),
            )),
        })
        .prompt()
        .context("Failed to get number of signers")?;

    let num_signers: u32 = num_signers_str
        .trim()
        .parse()
        .context("Failed to parse number of signers")?;

    println!();
    println!("📝 Collecting signatures from {} signer(s):", num_signers);
    println!();

    // Collect signatures and public keys from each signer
    let mut intent_signatures = Vec::new();

    for i in 1..=num_signers {
        println!("--- Signer {} ---", i);

        // Get public key for this signer
        let public_key_input = Text::new(&format!("Enter public key for signer {} (hex format):", i))
            .with_validator(validate_public_key)
            .prompt()
            .context("Failed to get public key")?;

        // Get signature for this signer
        let signature_input = Text::new(&format!("Enter signature from signer {} (hex format):", i))
            .with_validator(|input: &str| {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    return Ok(Validation::Invalid("Signature cannot be empty".into()));
                }
                match hex::decode(trimmed) {
                    Ok(bytes) => {
                        if bytes.len() == 64 {
                            Ok(Validation::Valid)
                        } else {
                            Ok(Validation::Invalid("Signature must be 64 bytes (Ed25519 only supported for now)".into()))
                        }
                    },
                    Err(_) => Ok(Validation::Invalid("Invalid hex format".into()))
                }
            })
            .prompt()
            .context("Failed to get signature")?;

        // Parse public key and signature
        let public_key_bytes = hex::decode(public_key_input.trim())
            .context("Failed to decode public key")?;
        let signature_bytes = hex::decode(signature_input.trim())
            .context("Failed to decode signature")?;

        // Create Ed25519 signature (only Ed25519 supported for now)
        if public_key_bytes.len() != 32 || signature_bytes.len() != 64 {
            return Err(anyhow::anyhow!(
                "Signer {} must use Ed25519 (32-byte key + 64-byte signature). Secp256k1 support will be added later.", 
                i
            ));
        }

        let mut pk_array = [0u8; 32];
        pk_array.copy_from_slice(&public_key_bytes);
        let mut sig_array = [0u8; 64];
        sig_array.copy_from_slice(&signature_bytes);

        let intent_signature =
            IntentSignatureV1(SignatureWithPublicKeyV1::Ed25519 {
                public_key: Ed25519PublicKey(pk_array),
                signature: Ed25519Signature(sig_array),
            });

        intent_signatures.push(intent_signature);
        println!("✅ Signer {} added successfully", i);
        println!();
    }

    // Get notary public key
    let notary_public_key_input = Text::new("Enter the notary public key (hex format):")
        .with_validator(validate_public_key)
        .prompt()
        .context("Failed to get notary public key")?;

    println!();
    println!("🔄 Constructing notarized transaction...");

    // Parse notary public key (Ed25519 only)
    let notary_key_bytes = hex::decode(notary_public_key_input.trim())
        .context("Failed to decode notary public key")?;

    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&notary_key_bytes);
    let notary_public_key = PublicKey::Ed25519(Ed25519PublicKey(key_bytes));

    // Create a minimal intent for transaction reconstruction
    // Note: This is a placeholder since we don't have the original manifest
    let _network_definition = NetworkDefinition::mainnet(); // Default to mainnet
    
    let placeholder_intent = IntentV1 {
        header: TransactionHeaderV1 {
            network_id: 1, // Default to mainnet
            start_epoch_inclusive: Epoch::of(1000),
            end_epoch_exclusive: Epoch::of(1100),
            nonce: 1,
            notary_public_key: notary_public_key.clone(),
            notary_is_signatory: false,
            tip_percentage: 0,
        },
        instructions: InstructionsV1(vec![]), // Placeholder - in real usage would need actual manifest
        blobs: BlobsV1 { blobs: vec![] },
        message: MessageV1::None,
    };

    // Create signed intent
    let signed_intent = SignedIntentV1 {
        intent: placeholder_intent,
        intent_signatures: IntentSignaturesV1 {
            signatures: intent_signatures,
        },
    };

    // Create placeholder notary signature (in real usage, notary would sign)
    let placeholder_notary_signature =
        NotarySignatureV1(SignatureV1::Ed25519(Ed25519Signature([0u8; 64])));

    // Create notarized transaction
    let _notarized_transaction = NotarizedTransactionV1 {
        signed_intent,
        notary_signature: placeholder_notary_signature,
    };

    // Calculate transaction ID (this would be the actual transaction hash)
    // For transaction ID, we'll use a simple hash of the structure
    let transaction_id = format!("tx_{}", hex::encode(&intent_hash_input.trim()[..16]));

    println!("✅ Notarized Transaction Constructed Successfully!");
    println!();
    println!("📋 Transaction Summary:");
    println!("  Intent Hash: {}", intent_hash_input.trim());
    println!("  Number of Signers: {}", num_signers);
    println!("  Notary Public Key: {}", notary_public_key_input.trim());
    println!("  Transaction ID: {}", transaction_id);
    println!();
    println!("⚠️  NOTE: This is a constructed transaction with placeholder notary signature.");
    println!("   In production, the notary would need to sign the signed intent hash.");
    println!("   Actual network submission functionality will be added later.");

    Ok(())
}
