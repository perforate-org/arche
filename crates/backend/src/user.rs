use candid::{CandidType, Deserialize, Decode, Encode};
use ic_stable_structures::storable::{Bound, Storable};
use std::borrow::Cow;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum User {
    V1(UserV1),
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UserV1 {
    pub name: String,
}
