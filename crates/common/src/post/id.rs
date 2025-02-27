use candid::CandidType;
use serde::{Serialize, Deserialize};
use derive_more::{AsRef, Display, From, Into};
use hexaurl::HexaUrl;

/// Wrapper for post ids.
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
pub struct PostId(HexaUrl);

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for PostId {
        fn to_bytes(&self) -> Cow<[u8]> {
            self.0.to_bytes()
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            PostId(HexaUrl::from_bytes(bytes))
        }

        const BOUND: Bound = HexaUrl::BOUND;
    }
}
