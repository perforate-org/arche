use candid::CandidType;
use derive_more::{AsRef, Display, From, Into};
use hexaurl::HexaUrl;
use serde::{Deserialize, Serialize};

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
    Display,
    AsRef,
    From,
    Into,
)]
#[as_ref(forward)]
pub struct PostId(HexaUrl);

impl PostId {
    /// Creates a new PostId from a string
    pub fn new(id: &str) -> Result<Self, &'static str> {
        Ok(Self(HexaUrl::new(id).map_err(|_| "Invalid post ID")?))
    }

    /// Generates a new unique PostId
    pub fn generate() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self(HexaUrl::new(&format!("post_{}", now)).unwrap())
    }
}

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
