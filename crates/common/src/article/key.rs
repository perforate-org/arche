use candid::CandidType;
use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};

/// Wrapper for article keys.
#[derive(
    CandidType,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Debug,
    Hash,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Display,
    AsRef,
    From,
    Into,
)]
#[as_ref(forward)]
pub struct ArticleKey(u64);

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for ArticleKey {
        fn to_bytes(&self) -> Cow<[u8]> {
            self.0.to_bytes()
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            ArticleKey(u64::from_bytes(bytes))
        }

        const BOUND: Bound = u64::BOUND;
    }
}
