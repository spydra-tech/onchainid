use soroban_sdk::{Address, Bytes, Env, String, Vec, U256};

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
    fn add_claim(e: Env, topic: U256, scheme: U256, issuer: Address, signature: Bytes, data: Bytes, uri: String) -> Bytes;

    /**
     * Get a claim by its ID.
     *
     * Claim IDs are generated using `keccak256(abi.encode(address issuer_address, uint256 topic))`.
     */
    fn get_claim(e: Env, claim_id: Bytes) -> (U256, U256, Address, Bytes, Bytes, String);

    /**
     * Removes a claim.
     *
     * Triggers Event: `ClaimRemoved`
     *
     * Claim IDs are generated using `keccak256(address issuer_address, uint256 topic)`.
     */
    fn remove_claim(e: Env, claim_id: Bytes) -> bool;

    /**
     * Returns an array of claim IDs by topic.
     */
    fn get_claim_ids_by_topic(e: Env, topic: U256) -> Vec<Bytes>;
}
