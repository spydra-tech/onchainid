use soroban_sdk::{BytesN, Env, Vec};

pub trait IERC734 {
    /**
     * Adds a _key to the identity. The _purpose specifies the purpose of the key.
     *
     * Triggers Event: `KeyAdded`
     *
     * Specification: MUST only be done by keys of purpose 1, or the identity
     * itself. If it's the identity itself, the approval process will determine its approval.
     */
    fn add_key(e: Env, key: BytesN<32>, purpose: u128, key_type: u128) -> bool;

    /**
     * Removes _purpose for _key from the identity.
     *
     * Triggers Event: `KeyRemoved`
     *
     * Specification: MUST only be done by keys of purpose 1, or the identity itself.
     * If it's the identity itself, the approval process will determine its approval.
     */
    fn remove_key(e: Env, key: BytesN<32>, purpose: u128) -> bool;

    /**
     * Returns the full key data, if present in the identity.
     */
    fn get_key(e: Env, key: BytesN<32>) -> (Vec<u128>, u128, BytesN<32>);

    /**
    * See {IERC734-getKeyPurposes}.
    * gets the purposes of a key
    * @param _key The public key
    * @return _purposes Returns the purposes of the specified key
    */
    fn get_key_purposes(e: Env, key: BytesN<32>) -> Vec<u128>;

    /**
     * Returns an array of public key held by this identity.
     */
    fn get_keys_by_purpose(e: Env, purpose: u128) -> Vec<BytesN<32>>;
    /**
     * Returns TRUE if a key is present and has the given purpose. If the key is not present it returns FALSE.
     */
    fn key_has_purpose(e: Env, key: BytesN<32>, purpose: u128) -> bool;
}