# OnchainID - Stellar Smart Contracts

[ONCHAINID](https://www.onchainid.com/) is a blockchain-based identity ecosystem that identifies individuals, organizations and DeFi protocols, allowing them to enforce compliance and access digital assets. OnchainID is built on top of several standards, mainly ERC734, and ERC735.

This project implements the smart contracts necessary to support OnchainId on Stellar Blockchain using Rust. The contracts are based on the  implementation in Solidity which can be found [here](https://github.com/onchain-id/solidity).

## How does it work?
The OnchainID protocol allows users to create their self-sovereign identity on the blockchain. The identity is a smart contract on the Blockchain published by a user which by itself has no value. It is the information (claims) attached to it that gives credit to the identity. This information can be self-attested, or signed on the blockchain by a trusted third-party such as a bank, a digital national identity key, a digital asset marketplace, a transfer agent, an auditor, etc. Such trusted third-parties create identity proofs that can be used by token issuers, custodians, DeFi protocols, etc. User's private data is kept off-chain with trusted parties, and encrypted identity proofs of data validation are published on the blockchain. Therefore, everyone knows that a trusted third party has successfully checked the identity. But to access the data, one would need the explicit consent of the Identity Owner
allowing the consultation of such private data. If the signature attesting the proof of identity is not issued by a credible entity in the opinion of someone who needs the relevant data, it would be possible for such person to do its own checks directly with the identity owner, as the identity
owner can share the relevant information with any person by giving them simple access to its OnchainID.

![How OnChainID works](https://assets-global.website-files.com/60ed5607a0d4556dd864b950/6101117227201d112e77dd7f_How%2520it%2520works%2520-%2520V3%25402x-p-1600.png)

## Who should use OnchainID?
OnchainID is an open source system open to everyone.Individuals, businesses, decentralized autonomous organizations and governments can use the system to obtain identities, certify information and use proofs of identity.

Asset owners
- Security owners and any type of financial instrument such as debt, equity, funds and real estate that needs to comply to regulations.
- NFTs that certify and authenticate ownership in digital and real world items.

Investors
- Institutional investors can use it to ensure that they are compliant to regulations.
- Everyday investors to access the ecosystem of regulated assets.

Custody
- To immobilize assets and represent them on the blockchain.

KYC/AML solutions
- To enable issuers to verify their investors.

DeFi protocols
- To comply with their regulatory obligations.

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── onchain_id
│       ├── src
│       │   ├── lib.rs
│       │   └── identity.rs
│       │   └── erc734
│       │   └── erc735
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

## Getting Started

- Setup the Stellar development environment as documented [here](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup).
- Build the contract
    ```console
    stellar contract build
    ```
- Deploy the contract to the Stellar network. For e.g. to deploy to the testnet with account alice,
    ```console
    stellar contract deploy `
        --wasm target/wasm32-unknown-unknown/release/onchain_id.wasm `
        --source alice `
        --network testnet
This will deploy the identity contract for alice on the testnet.
- Invoke the required smart contract methods. For e.g. to add a signing key (of a claim issuer to the contract), invoke the add_key method.
    ```console
    stellar contract invoke `
        --id CDY7H4U4GOZO3XQQSH7NUC7WN6WT6MCHI6NXUG6TOBFBNA2RVGK7ZZVU `
        --source alice `
        --network testnet `
        -- `
        add_key `
        --key GDFFVSBWPLQCSNOJD5LH4C24FDQA2YUZUF6AAYQOJZALSB2LCINEVRBJ `
        --purpose 3 `
        --key_type 1
    ```
    where id is the deployed identity contract address and key is the public key of the claims issuer.