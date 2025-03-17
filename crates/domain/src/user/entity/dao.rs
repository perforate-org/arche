use candid::{CandidType, Deserialize};

mod v1;
pub(crate) use v1::V1;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UserDao {
    pub(crate) version: UserDaoVersion,
}

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for UserDao {
        fn to_bytes(&self) -> Cow<[u8]> {
            Cow::Owned(candid::encode_one(self).unwrap())
        }
        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            candid::decode_one(bytes.as_ref()).unwrap()
        }
        const BOUND: Bound = Bound::Unbounded;
    }
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub(crate) enum UserDaoVersion {
    V1(V1),
}
