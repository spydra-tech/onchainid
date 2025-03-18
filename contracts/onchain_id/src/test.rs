#![cfg(test)]
extern crate std;

use crate::identity::{Identity, IdentityClient};
use soroban_sdk::{ testutils::BytesN as _, vec, Bytes, BytesN, Env, String};
use ed25519_dalek::{Keypair, Signer};
use rand::thread_rng;

#[test]
fn test_add_key() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Identity);
    let client = IdentityClient::new(&env, &contract_id);

    // Add issuer key with purpose as claim signer - purpose 3. Key type 1 is ECDSA
    let issuer = BytesN::random(&env);
    let add_key_result = client.add_key(&issuer, &3, &1);
    assert_eq!(add_key_result, true);
}

#[test]
fn test_get_key() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Identity);
    let client = IdentityClient::new(&env, &contract_id);

    // Add issuer key with purpose as claim signer - purpose 3. Key type 1 is ECDSA
    let issuer = BytesN::random(&env);
    let add_key_result = client.add_key(&issuer, &3, &1);
    assert_eq!(add_key_result, true);

    let get_key_result = client.get_key(&issuer);
    assert_eq!(get_key_result, (vec![&env, 3], 1, issuer.clone()));
}

#[test]
fn test_get_key_purposes() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Identity);
    let client = IdentityClient::new(&env, &contract_id);

    // Add issuer key with purpose as claim signer - purpose 3. Key type 1 is ECDSA
    let issuer = BytesN::random(&env);
    let add_key_result = client.add_key(&issuer, &3, &1);
    assert_eq!(add_key_result, true);

    let get_key_purposes_result = client.get_key_purposes(&issuer);
    assert_eq!(get_key_purposes_result, vec![&env, 3]);
}

#[test]
fn test_get_keys_by_purpose() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Identity);
    let client = IdentityClient::new(&env, &contract_id);

    // Add issuer key with purpose as claim signer - purpose 3. Key type 1 is ECDSA
    let issuer = BytesN::random(&env);
    let add_key_result = client.add_key(&issuer, &3, &1);
    assert_eq!(add_key_result, true);

    let get_keys_by_purpose_result = client.get_keys_by_purpose(&3);
    assert_eq!(get_keys_by_purpose_result, vec![&env, issuer.clone()]);
}

#[test]
fn test_key_has_purpose() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Identity);
    let client = IdentityClient::new(&env, &contract_id);

    // Add issuer key with purpose as claim signer - purpose 3. Key type 1 is ECDSA
    let issuer = BytesN::random(&env);
    let add_key_result = client.add_key(&issuer, &3, &1);
    assert_eq!(add_key_result, true);

    let key_has_purpose_result = client.key_has_purpose(&issuer, &3);
    assert_eq!(key_has_purpose_result, true);
}

#[test]
fn test_remove_key() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Identity);
    let client = IdentityClient::new(&env, &contract_id);

    // Add issuer key with purpose as claim signer - purpose 3. Key type 1 is ECDSA
    let issuer = BytesN::random(&env);
    let add_key_result = client.add_key(&issuer, &3, &1);
    assert_eq!(add_key_result, true);

    let remove_key_result = client.remove_key(&issuer, &3);
    assert_eq!(remove_key_result, true);
}

#[test]
fn test_add_claim() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Identity);
    let client = IdentityClient::new(&env, &contract_id);

    // Add issuer key with purpose as claim signer - purpose 3. Key type 1 is ECDSA
    let issuer_key_pair = Keypair::generate(&mut thread_rng());
    let issuer = BytesN::from_array(&env, issuer_key_pair.public.as_bytes());
    let add_key_result = client.add_key(&issuer, &3, &1);
    assert_eq!(add_key_result, true);

    //Add KYC claim
    let data = Bytes::from_slice(&env, "true".as_bytes());
    let identity_key: BytesN<32> = BytesN::random(&env);

    let mut combined = Bytes::new(&env);
    combined.append(&Bytes::from_slice(&env, &identity_key.to_array()));
    combined.append(&Bytes::from_slice(&env, &1010101_u32.to_be_bytes()));
    combined.append(&data);

    let combined_vec: std::vec::Vec<u8> = combined.iter().collect();
    let signature = issuer_key_pair.sign(&combined_vec).to_bytes();

    let claim_id = client.add_claim(&1010101, &1, &issuer, &BytesN::from_array(&env,&signature), &data, &String::from_str(&env, ""));

    let get_claim_result = client.get_claim(&claim_id);
    assert_eq!(get_claim_result, (1010101, 1, issuer.clone(), BytesN::from_array(&env,&signature), data, String::from_str(&env, "")));
}

#[test]
fn test_get_claim_ids_by_topic() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Identity);
    let client = IdentityClient::new(&env, &contract_id);

    // Add issuer key with purpose as claim signer - purpose 3. Key type 1 is ECDSA
    let issuer_key_pair = Keypair::generate(&mut thread_rng());
    let issuer = BytesN::from_array(&env, issuer_key_pair.public.as_bytes());
    let add_key_result = client.add_key(&issuer, &3, &1);
    assert_eq!(add_key_result, true);

    //Add KYC claim
    let data = Bytes::from_slice(&env, "true".as_bytes());
    let identity_key: BytesN<32> = BytesN::random(&env);

    let mut combined = Bytes::new(&env);
    combined.append(&Bytes::from_slice(&env, &identity_key.to_array()));
    combined.append(&Bytes::from_slice(&env, &1010101_u32.to_be_bytes()));
    combined.append(&data);

    let combined_vec: std::vec::Vec<u8> = combined.iter().collect();
    let signature = issuer_key_pair.sign(&combined_vec).to_bytes();

    let claim_id = client.add_claim(&1010101, &1, &issuer, &BytesN::from_array(&env,&signature), &data, &String::from_str(&env, ""));

    let get_claim_ids_by_topic_result = client.get_claim_ids_by_topic(&1010101);
    assert_eq!(get_claim_ids_by_topic_result, vec![&env, claim_id]);
}

#[test]
fn test_remove_claim() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Identity);
    let client = IdentityClient::new(&env, &contract_id);

    // Add issuer key with purpose as claim signer - purpose 3. Key type 1 is ECDSA
    let issuer_key_pair = Keypair::generate(&mut thread_rng());
    let issuer = BytesN::from_array(&env, issuer_key_pair.public.as_bytes());
    let add_key_result = client.add_key(&issuer, &3, &1);
    assert_eq!(add_key_result, true);

    //Add KYC claim
    let data = Bytes::from_slice(&env, "true".as_bytes());
    let identity_key: BytesN<32> = BytesN::random(&env);

    let mut combined = Bytes::new(&env);
    combined.append(&Bytes::from_slice(&env, &identity_key.to_array()));
    combined.append(&Bytes::from_slice(&env, &1010101_u32.to_be_bytes()));
    combined.append(&data);

    let combined_vec: std::vec::Vec<u8> = combined.iter().collect();
    let signature = issuer_key_pair.sign(&combined_vec).to_bytes();

    let claim_id = client.add_claim(&1010101, &1, &issuer, &BytesN::from_array(&env,&signature), &data, &String::from_str(&env, ""));

    let remove_claim_result = client.remove_claim(&claim_id);
    assert_eq!(remove_claim_result, true);
}

#[test]
fn is_claim_valid() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Identity);
    let client = IdentityClient::new(&env, &contract_id);

    // Add issuer key with purpose as claim signer - purpose 3. Key type 1 is ECDSA
    let issuer_key_pair = Keypair::generate(&mut thread_rng());
    let issuer = BytesN::from_array(&env, issuer_key_pair.public.as_bytes());
    let add_key_result = client.add_key(&issuer, &3, &1);
    assert_eq!(add_key_result, true);

    //Add KYC claim
    let data = Bytes::from_slice(&env, "true".as_bytes());
    let identity_key: BytesN<32> = BytesN::random(&env);

    let mut combined = Bytes::new(&env);
    combined.append(&Bytes::from_slice(&env, &identity_key.to_array()));
    combined.append(&Bytes::from_slice(&env, &1010101_u32.to_be_bytes()));
    combined.append(&data);

    let combined_vec: std::vec::Vec<u8> = combined.iter().collect();
    let signature = issuer_key_pair.sign(&combined_vec).to_bytes();

    let _claim_id = client.add_claim(&1010101, &1, &issuer, &BytesN::from_array(&env,&signature), &data, &String::from_str(&env, ""));

    let is_claim_valid_result = client.is_claim_valid(&identity_key, &issuer, &1010101, &BytesN::from_array(&env,&signature), &data);
    assert_eq!(is_claim_valid_result, true);
}