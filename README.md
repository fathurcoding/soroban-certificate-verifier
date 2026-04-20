# VeriChain - Soroban Certificate Verifier

A decentralized certificate verification system built on the Stellar Soroban network. VeriChain allows users to securely register, validate, and retrieve digital certificates using SHA-256 hashes, ensuring data integrity and authenticity.

![Testnet Application Screenshot](Screenshot%202026-04-20%20114056.png)

## Features

- **Register Certificate**: Users can register a document's SHA-256 hash along with a title (up to 50 characters). The contract automatically records the owner's address and the current ledger timestamp to prevent duplicate hash registrations.
- **Validate Certificate**: Allows anyone to check if a specific certificate hash exists on the blockchain, quickly verifying its authenticity.
- **Get Certificate Record**: Retrieves detailed information about a registered certificate, including its owner's address, registration timestamp, and the certificate title.
- **Get User Certificates**: Fetches a list of all certificate hashes registered by a specific user address.

## Smart Contract Details

- **Testnet Contract ID**: `CB7FLDEWHCJ2DLM57ANMLJ7RCHH5R23R76IJDBC74EY4V6E4UT727WJA`

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello-world (VeriChain Contract)
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- Contracts are located in the `contracts` directory. The VeriChain smart contract logic is implemented inside the `contracts/hello-world/src/lib.rs` file.
- Contracts have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
