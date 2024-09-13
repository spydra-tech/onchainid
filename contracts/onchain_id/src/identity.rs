use soroban_sdk::{contract, contractimpl, crypto::Crypto, vec, xdr::ToXdr, Address, Bytes, Env, String, Vec, U256};
use crate::{claims_issuer::traits::IClaimIssuer, erc734::traits::IERC734, erc735::traits::IERC735, structs::{Claim, DataKey, Key}};

#[contract]
pub struct Identity;

#[contractimpl]
impl IERC734 for Identity {
    /**
    * implementation of the addKey function of the ERC-734 standard
    * Adds a _key to the identity. The _purpose specifies the purpose of key. Initially we propose four purposes:
    * 1: MANAGEMENT keys, which can manage the identity
    * 2: ACTION keys, which perform actions in this identities name (signing, logins, transactions, etc.)
    * 3: CLAIM signer keys, used to sign claims on other identities which need to be revokable.
    * 4: ENCRYPTION keys, used to encrypt data e.g. hold in claims.
    * MUST only be done by keys of purpose 1, or the identity itself.
    * If its the identity itself, the approval process will determine its approval.
    * @param _key public key
    * @param _type type of key used, which would be a uint256 for different key types. e.g. 1 = ECDSA, 2 = RSA, etc.
    * @param _purpose a uint256 specifying the key type, like 1 = MANAGEMENT, 2 = ACTION, 3 = CLAIM, 4 = ENCRYPTION
    * @return success Returns TRUE if the addition was successful and FALSE if not
    */
    fn add_key(e: Env, key: String, purpose: u128, key_type: u128) -> bool {
        //TODO: Authorization check for only manager

        let map_key = DataKey::Key(key.clone());
        if let Some(mut retrieved_key) = e.storage().persistent().get::<DataKey, Key>(&map_key) {
            let retrieved_purposes: Vec<u128> = retrieved_key.purposes.clone();

            for retrieved_purpose in retrieved_purposes {
                if purpose == retrieved_purpose {
                    panic!("Conflict: Key already has purpose");
                }
            }

            retrieved_key.purposes.push_back(purpose);
        } else {
            let new_purposes: Vec<u128> = vec![&e, purpose];
            let new_key: Key = Key {
                purposes: new_purposes,
                key_type: key_type,
                key: key.clone(),
            };
            e.storage().persistent().set(&map_key, &new_key);

            //TODO: Set TTL?
        }

        let purpose_key = DataKey::Purpose(purpose);
        if let Some(mut retrieved_purpose_keys) = e.storage().persistent().get::<DataKey, Vec<String>>(&purpose_key) {
            retrieved_purpose_keys.push_back(key);
            e.storage().persistent().set(&purpose_key, &retrieved_purpose_keys);
        } else {
            let retrieved_purpose_keys = vec![&e, key];
            e.storage().persistent().set(&purpose_key, &retrieved_purpose_keys);
        }
  
        //TODO: Emit Event
        return true;
    }

    /**
    * See {IERC734-removeKey}.
    * Remove the purpose from a key.
    */
    fn remove_key(e: Env, key: String, purpose: u128) -> bool {
        let map_key = DataKey::Key(key.clone());
        if let Some(mut retrieved_key) = e.storage().persistent().get::<DataKey, Key>(&map_key) {
            let mut retrieved_purposes = retrieved_key.purposes;
            let mut purpose_index = 0;
            while retrieved_purposes.get_unchecked(purpose_index) != purpose {
                purpose_index = purpose_index+1;
                if purpose_index == retrieved_purposes.len() {
                    panic!("NotFound: Key doesn't have such purpose");
                }
            }

            retrieved_purposes.set(purpose_index, retrieved_purposes.get_unchecked(retrieved_purposes.len()-1));
            retrieved_purposes.pop_back();
            retrieved_key.purposes = retrieved_purposes;

            if retrieved_key.purposes.len()-1 == 0 {
                e.storage().persistent().remove(&map_key);
            } else {
                e.storage().persistent().set(&map_key, &retrieved_key);
            }

            let mut key_index = 0;
            let map_purpose = DataKey::Purpose(purpose);
            if let Some(mut retrieved_keys) = e.storage().persistent().get::<DataKey, Vec<String>>(&map_purpose) {
                let array_length = retrieved_keys.len();

                while retrieved_keys.get_unchecked(key_index) != key {
                    key_index = key_index+1;

                    if key_index >= array_length {
                        break;
                    }
                }

                retrieved_keys.set(key_index, retrieved_keys.get_unchecked(array_length-1));
                retrieved_keys.pop_back();
                e.storage().persistent().set(&map_purpose, &retrieved_keys);
                //TODO: Adjust TTL
            }

            //TODO: Raise Event
            return true;
        } else {
            panic!("NotFound: Key isn't registered");
        }
    }

    /**
     * See {IERC734-getKey}.
     * Implementation of the getKey function from the ERC-734 standard
     * @param _key The public key. 
     * @return purposes Returns the full key data, if present in the identity.
     * @return keyType Returns the full key data, if present in the identity.
     * @return key Returns the full key data, if present in the identity.
     */
    fn get_key(e: Env, key: String) -> (Vec<u128>, u128, String){
        let map_key = DataKey::Key(key.clone());
        if let Some(retrieved_key) = e.storage().persistent().get::<DataKey, Key>(&map_key) {
            (retrieved_key.purposes, retrieved_key.key_type, retrieved_key.key)
        } else {
            panic!("NotFound: Key isn't registered");
        }
    }

    /**
    * See {IERC734-getKeyPurposes}.
    * gets the purposes of a key
    * @param _key The public key
    * @return _purposes Returns the purposes of the specified key
    */
    fn get_key_purposes(e: Env, key: String) -> Vec<u128>{
        let map_key = DataKey::Key(key.clone());
        if let Some(retrieved_key) = e.storage().persistent().get::<DataKey, Key>(&map_key) {
            retrieved_key.purposes
        } else {
            panic!("NotFound: Key isn't registered");
        }
    }

    /**
    * See {IERC734-getKeysByPurpose}.
    * gets all the keys with a specific purpose from an identity
    * @param _purpose a uint256[] Array of the key types, like 1 = MANAGEMENT, 2 = ACTION, 3 = CLAIM, 4 = ENCRYPTION
    * @return keys Returns an array of public key held by this identity and having the specified purpose
    */
    fn get_keys_by_purpose(e: Env, purpose: u128) -> Vec<String>{
        let map_purpose = DataKey::Purpose(purpose);
        if let Some(retrieved_key) = e.storage().persistent().get::<DataKey, Vec<String>>(&map_purpose) {
            return retrieved_key;
        } else {
            return vec![&e];
        }
    }

    fn key_has_purpose(e: Env, key: String, purpose: u128) -> bool{
        let map_key = DataKey::Key(key.clone());
        if let Some(retrieved_key) = e.storage().persistent().get::<DataKey, Key>(&map_key) {
            let retrieved_purposes: Vec<u128> = retrieved_key.purposes;

            for retrieved_purpose in retrieved_purposes {
                if purpose == 1 || purpose == retrieved_purpose {
                    return true;
                }
            }
        } else {
            panic!("NotFound: Key isn't registered");
        }

        return false;
    }
}  

#[contractimpl]
impl IERC735 for Identity {
    fn add_claim(e: Env, topic: U256, scheme: U256, issuer: Address, signature: Bytes, data: Bytes, uri: String) -> Bytes{
        //TODO: Check claim key authorization
        //Hash the concatenated value below and check that the signature is valid.

        let mut claim_id = Bytes::new(&e);
        claim_id.append(&issuer.clone().to_xdr(&e));
        claim_id.append(&topic.clone().to_xdr(&e));

        let map_key = DataKey::Claim(claim_id.clone());
        if let Some(mut retrieved_claim) = e.storage().persistent().get::<DataKey, Claim>(&map_key) {
            retrieved_claim.topic = topic;
            retrieved_claim.scheme = scheme;
            retrieved_claim.signature = signature;
            retrieved_claim.data = data;
            retrieved_claim.uri = uri;

            e.storage().persistent().set(&map_key, &retrieved_claim);
        } else {
            let claim: Claim = Claim {
                topic: topic.clone(),
                scheme,
                issuer,
                signature,
                data,
                uri
            };
            e.storage().persistent().set(&map_key, &claim);

            let claim_topic_key = DataKey::ClaimTopic(topic.clone());
            if let Some(mut retrieved_claim_topics) = e.storage().persistent().get::<DataKey, Vec<Bytes>>(&claim_topic_key) {
                retrieved_claim_topics.push_back(claim_id.clone());
                e.storage().persistent().set(&claim_topic_key, &retrieved_claim_topics);
            } else {
                let retrieved_claim_topics = vec![&e, topic];
                e.storage().persistent().set(&claim_topic_key, &retrieved_claim_topics);
            }
        }

        return claim_id;
    }

    /**
    * See {IERC735-getClaim}.
    * Implementation of the getClaim function from the ERC-735 standard.
    *
    * @param _claimId The identity of the claim i.e. keccak256(abi.encode(_issuer, _topic))
    *
    * @return topic Returns all the parameters of the claim for the
    * specified _claimId (topic, scheme, signature, issuer, data, uri) .
    * @return scheme Returns all the parameters of the claim for the
    * specified _claimId (topic, scheme, signature, issuer, data, uri) .
    * @return issuer Returns all the parameters of the claim for the
    * specified _claimId (topic, scheme, signature, issuer, data, uri) .
    * @return signature Returns all the parameters of the claim for the
    * specified _claimId (topic, scheme, signature, issuer, data, uri) .
    * @return data Returns all the parameters of the claim for the
    * specified _claimId (topic, scheme, signature, issuer, data, uri) .
    * @return uri Returns all the parameters of the claim for the
    * specified _claimId (topic, scheme, signature, issuer, data, uri) .
    */
    fn get_claim(e: Env, claim_id: Bytes) -> (U256, U256, Address, Bytes, Bytes, String){
        let map_key = DataKey::Claim(claim_id.clone());
        if let Some(retrieved_claim) = e.storage().persistent().get::<DataKey, Claim>(&map_key) {
            (retrieved_claim.topic, retrieved_claim.scheme, retrieved_claim.issuer, retrieved_claim.signature, retrieved_claim.data, retrieved_claim.uri)
        } else {
            panic!("NotFound: There is no claim with this ID");
        }
    }

    /**
    * See {IERC735-removeClaim}.
    * Implementation of the removeClaim function from the ERC-735 standard
    * Can only be removed by the claim issuer, or the claim holder itself.
    *
    * @param _claimId The identity of the claim i.e. keccak256(_issuer, _topic)
    *
    * @return success Returns TRUE when the claim was removed.
    * triggers ClaimRemoved event
    */
    fn remove_claim(e: Env, claim_id: Bytes) -> bool{
        let map_key = DataKey::Claim(claim_id.clone());
        if let Some(retrieved_claim) = e.storage().persistent().get::<DataKey, Claim>(&map_key) {
            e.storage().persistent().remove(&map_key);

            let topic_key = DataKey::ClaimTopic(retrieved_claim.topic);
            if let Some(mut retrieved_claim_topic) = e.storage().persistent().get::<DataKey, Vec<Bytes>>(&topic_key) {
                let mut claim_index = 0;
                for retrieved_claim_id in retrieved_claim_topic.clone() {
                    if retrieved_claim_id == claim_id {
                        retrieved_claim_topic.remove(claim_index);
                        break;
                    }
                    claim_index = claim_index + 1;
                }
            }

        } else {
            panic!("NotFound: There is no claim with this ID");
        }
        return true;
    }

    /**
    * See {IERC735-getClaimIdsByTopic}.
    * Implementation of the getClaimIdsByTopic function from the ERC-735 standard.
    * used to get all the claims from the specified topic
    * @param _topic The identity of the claim i.e. keccak256(_issuer, _topic)
    * @return claimIds Returns an array of claim IDs by topic.
    */
    fn get_claim_ids_by_topic(e: Env, topic: U256) -> Vec<Bytes>
    {
        let map_key = DataKey::ClaimTopic(topic.clone());
        if let Some(retrieved_claim_topic) = e.storage().persistent().get::<DataKey, Vec<Bytes>>(&map_key) {
            retrieved_claim_topic
        } else {
            panic!("NotFound: Claim topic not found");
        }
    }
}

#[contractimpl]
impl IClaimIssuer for Identity {

    /**
     * Checks if a claim is valid.
     */
    fn is_claim_valid(e: Env, identity: Address, topic: u128, sig: Bytes, data: Bytes) -> bool{
        //TODO: validate the signature here.
        return true;
    }
}