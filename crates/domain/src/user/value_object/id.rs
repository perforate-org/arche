use candid::CandidType;
use derive_more::{AsRef, Display, From, Into};
use hexaurl::HexaUrl;
use serde::{Deserialize, Serialize};

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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_creation() {
        // Valid user ID
        let valid_id = UserId::new("valid-user");
        assert!(valid_id.is_ok());

        // Invalid user ID (with spaces)
        let invalid_id = UserId::new("invalid user id");
        assert!(invalid_id.is_err());
    }

    #[test]
    fn test_user_id_display() {
        let user_id = UserId::new("test-user").unwrap();
        assert_eq!(format!("{}", user_id), "test-user");
    }

    #[test]
    fn test_user_id_equality() {
        let id1 = UserId::new("same-id").unwrap();
        let id2 = UserId::new("same-id").unwrap();
        let id3 = UserId::new("different-id").unwrap();

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}
