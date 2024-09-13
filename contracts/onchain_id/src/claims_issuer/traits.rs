use soroban_sdk::{Address, Bytes, Env};

pub trait IClaimIssuer {

    /**
     * Checks if a claim is valid.
     * @param _identity the identity contract related to the claim
     * @param claimTopic the claim topic of the claim
     * @param sig the signature of the claim
     * @param data the data field of the claim
     * @return claimValid true if the claim is valid, false otherwise
     */
    fn is_claim_valid(e: Env, identity: Address, topic: u128, sig: Bytes, data: Bytes) -> bool;
}