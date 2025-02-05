use soroban_sdk::{Bytes, BytesN, Env, String, Vec};

use crate::error::OnChainIdError;

/**
     * Add or update a claim.
     *
     * Triggers Event: `ClaimAdded`, `ClaimChanged`
     *
     * Specification: Add or update a claim from an issuer.
     *
     * _signature is a signed message of the following structure:
     * `keccak256(address identityHolder_address, uint256 topic, bytes data)`.
     * Claim IDs are generated using `keccak256(address issuer_address + uint256 topic)`.
     */
pub trait IERC735 {
    /**
     * Add or update a claim.
     *
     * Triggers Event: `ClaimAdded`, `ClaimChanged`
     *
     * Specification: Add or update a claim from an issuer.
     *
     * _signature is a signed message of the following structure:
     * `keccak256(address identityHolder_address, uint256 topic, bytes data)`.
     * Claim IDs are generated using `keccak256(address issuer_address + uint256 topic)`.
     */
    fn add_claim(e: Env, topic: u32, scheme: u32, issuer: BytesN<32>, signature: BytesN<64>, data: Bytes, uri: String) -> BytesN<32>;

    /**
     * Get a claim by its ID.
     *
     * Claim IDs are generated using `keccak256(abi.encode(address issuer_address, uint256 topic))`.
     */
    fn get_claim(e: Env, claim_id: BytesN<32>) -> Result<(u32, u32, BytesN<32>, BytesN<64>, Bytes, String), OnChainIdError>;

    /**
     * Removes a claim.
     *
     * Triggers Event: `ClaimRemoved`
     *
     * Claim IDs are generated using `keccak256(address issuer_address, uint256 topic)`.
     */
    fn remove_claim(e: Env, claim_id: BytesN<32>) -> Result<bool, OnChainIdError>;

    /**
     * Returns an array of claim IDs by topic.
     */
    fn get_claim_ids_by_topic(e: Env, topic: u32) -> Result<Vec<BytesN<32>>, OnChainIdError>;
}
