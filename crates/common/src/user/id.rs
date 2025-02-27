use candid::CandidType;
use derive_more::{AsRef, Display, From, Into};
use hexaurl::HexaUrl;
use serde::{Serialize, Deserialize};

/// Wrapper for user ids.
#[derive(
    CandidType,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    Hash,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    AsRef,
    From,
    Display,
    Into,
)]
pub struct UserId(HexaUrl);

impl UserId {
    pub fn new(input: &str) -> Result<Self, hexaurl::Error> {
        Ok(UserId(HexaUrl::new(input)?))
    }
}

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for UserId {
        fn to_bytes(&self) -> Cow<[u8]> {
            self.0.to_bytes()
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            UserId(HexaUrl::from_bytes(bytes))
        }

        const BOUND: Bound = HexaUrl::BOUND;
    }
}
