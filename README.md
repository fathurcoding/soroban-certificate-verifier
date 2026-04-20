# Soroban Certificate Verifier

## Overview

This project implements a decentralized certificate verification system (**VeriChain**) on the Stellar Soroban network. The smart contract allows users to register, validate, and retrieve certificate hashes securely on the blockchain.

![Certificate Verifier Screenshot](./Screenshot%202026-04-20%20114056.png)

## Smart Contract Logic

The core logic is located in `contracts/hello-world/src/lib.rs`. It features the `VeriChain` contract with the following functionality:

### Data Structures
- **`CertRecord`**: Stores the metadata of a certificate, including the `owner` (Address), block `timestamp`, and certificate `title`.
- **`DataKey`**: Manages persistent storage by mapping a certificate hash (`Record(Bytes)`) to its `CertRecord`, and mapping an `Address` to a list of its registered hashes (`UserCerts(Address)`).

### Core Features
1. **`register_certificate`**: Authenticates the owner and registers a new certificate hash with a title (max 50 chars). It prevents duplicate registrations, stores the `CertRecord`, adds the hash to the user's certificate list, and publishes a `register` event.
2. **`validate_certificate`**: Quickly checks if a given certificate hash exists on-chain.
3. **`get_certificate_record`**: Retrieves the owner, timestamp, and title of a specific certificate hash.
4. **`get_user_certificates`**: Returns an array of all certificate hashes owned by a specific address.

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well.
