use candid::CandidType;
use serde::{Deserialize, Serialize};
use crate::user::UserPrimaryKey;

mod v1;
pub(crate) use v1::V1;

/// Represents a technical article in the system
#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
pub struct ArticleDao<K: UserPrimaryKey> {
    pub(crate) version: ArticleDaoVersion<K>,
}

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl<K> Storable for ArticleDao<K>
    where
        K: UserPrimaryKey + CandidType + for<'de> Deserialize<'de>,
    {
        fn to_bytes(&self) -> Cow<[u8]> {
            Cow::Owned(candid::encode_one(self).expect("Failed to encode ArticleDao"))
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            candid::decode_one(&bytes).expect("Failed to decode ArticleDao")
        }

        const BOUND: Bound = Bound::Unbounded;
    }
}

#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
pub(crate) enum ArticleDaoVersion<K: UserPrimaryKey> {
    V1(V1<K>),
}
