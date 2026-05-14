# Example Execution

Below is a complete example of using the multisign-cli to prepare, sign, and submit a transaction on stokenet.

```
cargo run --bin multisign-cli
   Compiling gateway_api v0.1.0 (/Users/shambu/project/trellisarch/radix-multisign-helper/gateway_api)
   Compiling multisign-cli v0.1.0 (/Users/shambu/project/trellisarch/radix-multisign-helper/multisign-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.17s
     Running `target/debug/multisign-cli`
🔐 Multisign Helper - Interactive Transaction Signing Tool

> Select a command: prepare-intent-hash
📝 Preparing Intent Hash from Transaction Manifest

> Enter the notary public key (hex format): a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5
b6c7d8e9f0a1b2
> Select notary key type: Ed25519
> Enter network ID: 2
> Enter epoch number (leave empty to fetch current epoch. If you are submitting a transaction, y
ou should use the epoch of the transaction that was prepared): 
> How would you like to provide the transaction manifest? Paste manifest content directly
> Enter the transaction manifest content: <received>
  Transaction ID: txid_tdx_2_1abc123def456ghi789jkl012mno345pqr678stu901vwx234yz567abc890
  Intent Hash: f0e1d2c3b4a5968778695a4b3c2d1e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d
  Notary Public Key: a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2
  Current Epoch: 210129

💡 Save this details including intent hash for signing by the required parties.

> Select a command: sign-intent-hash
✏️ Sign Intent Hash

> Enter the intent hash (hex format): f0e1d2c3b4a5968778695a4b3c2d1e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c
6d
> Select key type: Ed25519
> Enter your private key (hex format, Ed25519 32 bytes): ********

🔄 Signing...
✅ Signature Generated Successfully!

📋 Results:
  Intent Hash: f0e1d2c3b4a5968778695a4b3c2d1e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d
  Key Type: Ed25519
  Public Key: a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2
  Signature: 9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d7e6f5a4b3c2d1e0f9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d7e6f5a4b3c2d1e0f9a8b0102

💡 Use these values for transaction assembly.

> Select a command: submit-trxn
🚀 Submit Transaction

> Enter the intent hash (hex format): f0e1d2c3b4a5968778695a4b3c2d1e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c
6d
> Enter the notary public key (hex format): a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5
b6c7d8e9f0a1b2
> Select notary key type: Ed25519
> Enter the notary private key (hex format): ********
> Enter network ID: 2
> Enter epoch number (leave empty to fetch current epoch. If you are submitting a transaction, y
ou should use the epoch of the transaction that was prepared): 210129
> How would you like to provide the transaction manifest? Paste manifest content directly
> Enter the transaction manifest content: <received>
Constructed Intent Hash: f0e1d2c3b4a5968778695a4b3c2d1e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d
Passed Intent Hash: f0e1d2c3b4a5968778695a4b3c2d1e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d
Notary Public Key: a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2
Network Definition: NetworkDefinition { id: 2, logical_name: "stokenet", hrp_suffix: "tdx_2_" }
Current Epoch: 210129
> Enter the number of signers: 1

📝 Collecting signatures from 1 signer(s):

--- Signer 1 ---
> Enter public key for signer 1 (hex format): a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4
a5b6c7d8e9f0a1b2
> Enter signature from signer 1 (hex format): 9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d7e6f5a
4b3c2d1e0f9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d7e6f5a4b3c2d1e0f9a8b0102
✅ Signer 1 added successfully


🔄 Constructing notarized transaction...
✅ Notarized Transaction Constructed Successfully!

📋 Transaction Summary:
  Number of Signers: 1
  Notary Public Key: a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2
  Transaction ID: txid_tdx_2_1abc123def456ghi789jkl012mno345pqr678stu901vwx234yz567abc890

Compiled Transaction (hex): 4d220302210221042107...&lt;truncated for brevity&gt;...c105

> Do the transaction details above match what you expect and are you ready to submit? Yes, submi
t the transaction
🚀 Submitting transaction...
Transaction Successful txid_tdx_2_1abc123def456ghi789jkl012mno345pqr678stu901vwx234yz567abc890
✅ Transaction submitted successfully!

> Select a command: exit
Goodbye! 👋
```
