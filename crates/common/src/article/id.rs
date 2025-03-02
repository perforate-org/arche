use candid::CandidType;
use derive_more::{AsRef, Display, From, Into};
use hexaurl::HexaUrl;
use serde::{Deserialize, Serialize};

/// Wrapper for article ids.
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
pub struct ArticleId(HexaUrl);

impl ArticleId {
    /// Creates a new ArticleId from a string
    pub fn new(id: &str) -> Result<Self, &'static str> {
        Ok(Self(HexaUrl::new(id).map_err(|_| "Invalid article ID")?))
    }

    /// Generates a new unique ArticleId
    pub fn generate() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self(HexaUrl::new(&format!("article_{}", now)).unwrap())
    }
}

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for ArticleId {
        fn to_bytes(&self) -> Cow<[u8]> {
            self.0.to_bytes()
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            ArticleId(HexaUrl::from_bytes(bytes))
        }

        const BOUND: Bound = HexaUrl::BOUND;
    }
}
