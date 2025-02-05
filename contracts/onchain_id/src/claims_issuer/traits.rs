use soroban_sdk::{Bytes, BytesN, Env};

use crate::error::OnChainIdError;

pub trait IClaimIssuer {

    /**
     * Checks if a claim is valid.
     * @param _identity the identity contract related to the claim
     * @param claimTopic the claim topic of the claim
     * @param sig the signature of the claim
     * @param data the data field of the claim
     * @return claimValid true if the claim is valid, false otherwise
     */
    fn is_claim_valid(e: Env, identity: BytesN<32>, issuer: BytesN<32>, topic: u32, sig: BytesN<64>, data: Bytes) -> Result<bool, OnChainIdError>;
}