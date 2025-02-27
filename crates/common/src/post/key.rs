use candid::CandidType;
use serde::{Serialize, Deserialize};
use derive_more::{AsRef, Display, From, Into};

/// Wrapper for post keys.
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
)]
#[derive(Display, AsRef, From, Into)]
#[as_ref(forward)]
pub struct PostKey(u64);

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for PostKey {
        fn to_bytes(&self) -> Cow<[u8]> {
            self.0.to_bytes()
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            PostKey(u64::from_bytes(bytes))
        }

        const BOUND: Bound = u64::BOUND;
    }
}
