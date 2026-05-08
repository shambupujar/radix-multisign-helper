# Multisign Helper

Interactive CLI tool for preparing, signing, and submitting multi-signature transactions on the Radix network.

## Overview

Multisign Helper guides you through the full lifecycle of a multi-signature Radix transaction:

1. **Prepare** a transaction intent hash from a manifest
2. **Sign** the intent hash with each signer's private key
3. **Submit** the fully-signed transaction to the Radix Gateway

The tool supports both **Ed25519** and **Secp256k1** key types, and works with **mainnet**, **stokenet**, and **simulator** networks.

## Prerequisites

- Rust toolchain (stable)
- A Radix transaction manifest file (`.rtm`) or manifest content to paste

## Building

```bash
cd multisign-helper
cargo build --release
```

The binary will be at `target/release/multisign-cli`.

## Running

```bash
cargo run --bin multisign-cli
```

The tool presents an interactive menu with four options:

```
🔐 Multisign Helper - Interactive Transaction Signing Tool

> prepare-intent-hash
  sign-intent-hash
  submit-trxn
  exit
```

## Commands

### `prepare-intent-hash`

Creates a transaction intent hash from a manifest file. This is the first step — the resulting intent hash is what each signer will sign.

**Prompts:**

| Prompt | Description |
|--------|-------------|
| Notary public key | Hex-encoded public key (32 bytes for Ed25519, 33 bytes for Secp256k1) |
| Key type | `Ed25519` or `Secp256k1` |
| Network ID | `1` = mainnet, `2` = stokenet, other = simulator (default: `1`) |
| Epoch | Current epoch number, or leave empty to auto-fetch from the Gateway API |
| Transaction manifest | Path to an `.rtm` file, or paste the manifest content directly |

**Output:**

- Transaction ID (Bech32-encoded)
- Intent Hash (hex)
- Notary Public Key
- Current Epoch

### `sign-intent-hash`

Signs an intent hash with a private key. Run this once for each signer. This command is offline — it does not call the Gateway API.

**Prompts:**

| Prompt | Description |
|--------|-------------|
| Intent hash | Hex-encoded hash from the prepare step (exactly 32 bytes / 64 hex characters) |
| Key type | `Ed25519` or `Secp256k1` |
| Private key | Hex-encoded private key (32 bytes, input is masked) |

**Output:**

- Intent Hash
- Key Type
- Public Key (derived from the private key)
- Signature (hex)

### `submit-trxn`

Assembles all signatures, notarizes, and submits the transaction to the Radix Gateway API. Polls for transaction status up to 30 times (1 second intervals).

**Prompts:**

| Prompt | Description |
|--------|-------------|
| Intent hash | Same hex hash used during signing |
| Notary public key | Notary's hex-encoded public key |
| Notary private key | Notary's hex-encoded private key (masked) |
| Notary key type | `Ed25519` or `Secp256k1` |
| Transaction manifest | Same manifest used during prepare |
| Network ID / Epoch | Same values used during prepare |
| Number of signers | How many signatures to collect (1–20) |
| Per signer: public key | Hex-encoded public key (key type auto-detected from length) |
| Per signer: signature | Hex-encoded signature from the sign step |

**Output:**

- Transaction summary
- Compiled transaction hex
- Submission result and final transaction status

## Typical Workflow

A multi-signature transaction involves coordination between a **notary** and one or more **signers**.

### Step 1: Prepare the intent hash (notary)

```
cargo run --bin multisign-cli
> prepare-intent-hash

# Provide notary public key, network, epoch, and manifest
# Note down the Intent Hash from the output
```

### Step 2: Distribute the intent hash

Share the intent hash with all signers through a secure channel.

### Step 3: Each signer signs the intent hash

Each signer runs the tool independently:

```
cargo run --bin multisign-cli
> sign-intent-hash

# Paste the intent hash
# Select key type and enter private key
# Share the resulting public key + signature back to the notary
```

### Step 4: Submit the transaction (notary)

```
cargo run --bin multisign-cli
> submit-trxn

# Provide notary keys, manifest, and all signer public keys + signatures
# Confirm submission
# Wait for transaction status
```

## Network Configuration

| Network ID | Network | Gateway API URL |
|------------|---------|-----------------|
| `1` | Mainnet | `https://mainnet.radixdlt.com` |
| `2` | Stokenet | Stokenet gateway |
| Other | Simulator | Local simulator |

## Project Structure

```
multisign-helper/
├── Cargo.toml              # Workspace root
├── multisign-cli/          # CLI binary
│   └── src/
│       ├── main.rs         # Entry point and interactive menu
│       └── commands.rs     # Command implementations
└── gateway_api/            # Radix Gateway API client library
    └── src/
        ├── lib.rs          # Module exports
        ├── transaction.rs  # Transaction submit & status polling
        ├── status.rs       # Epoch / gateway status
        ├── preview.rs      # Transaction preview (unused by CLI)
        ├── entity_details.rs # Validator details (unused by CLI)
        └── utils.rs        # Constants and helpers
```

## Key Dependencies

- **radix-transactions** / **radix-common** / **scrypto** `1.3.0` — Transaction compilation and cryptographic signing
- **inquire** — Interactive CLI prompts
- **tokio** — Async runtime
- **reqwest** — HTTP client for Gateway API calls
