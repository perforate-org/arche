use candid::CandidType;
use derive_more::{AsRef, Display, Into};
use serde::{Deserialize, Serialize};

/// Wrapper for user names.
#[derive(
    CandidType,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Hash,
    Eq,
    PartialEq,
    AsRef,
    Display,
    Into,
)]
pub struct UserName(String);

impl UserName {
    pub fn new(input: &str) -> Result<Self, String> {
        if input.is_empty() {
            return Err("UserName cannot be empty".to_string());
        }

        if input.chars().count() > 50 {
            return Err("UserName cannot exceed 50 characters".to_string());
        }

        Ok(UserName(input.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for UserName {
        fn to_bytes(&self) -> Cow<[u8]> {
            self.0.to_bytes()
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            Self(String::from_bytes(bytes))
        }

        const BOUND: Bound = Bound::Bounded {
            max_size: 200,
            is_fixed_size: false,
        };
    }
}
