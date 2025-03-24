use candid::CandidType;
use serde::{Deserialize, Serialize};
use crate::user::UserPrimaryKey;

mod v1;
pub(crate) use v1::V1;

/// Represents a technical paper in the system
#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
pub struct PaperDao<K: UserPrimaryKey> {
    pub(crate) version: PaperDaoVersion<K>,
}

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl<K> Storable for PaperDao<K>
    where
        K: UserPrimaryKey + CandidType + for<'de> Deserialize<'de>,
    {
        fn to_bytes(&self) -> Cow<[u8]> {
            Cow::Owned(candid::encode_one(self).expect("Failed to encode PaperDao"))
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            candid::decode_one(&bytes).expect("Failed to decode PaperDao")
        }

        const BOUND: Bound = Bound::Unbounded;
    }
}

#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
pub(crate) enum PaperDaoVersion<K: UserPrimaryKey> {
    V1(V1<K>),
}
