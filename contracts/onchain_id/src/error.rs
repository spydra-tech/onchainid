use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum OnChainIdError {
    KeyNotRegistered = 1,
    KeyAlreadyHasPurpose = 2,
    KeyDoesntHavePurpose = 3,
    NoClaimFound = 4,
    NoClaimTopicFound = 5,
    IssuerKeyNotAuthorized = 6
}