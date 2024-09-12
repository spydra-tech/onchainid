use soroban_sdk::{Address, Bytes, Env, String, U256};

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
    fn add_claim(e: Env, topic: U256, scheme: U256, issuer: Address, signature: Bytes, data: Bytes, uri: String) -> String;
}