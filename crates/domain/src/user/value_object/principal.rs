use candid::{CandidType, Principal};
use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};

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
    AsRef,
    From,
    Display,
    Into,
)]
pub struct UserPrincipal(Principal);

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for UserPrincipal {
        fn to_bytes(&self) -> Cow<[u8]> {
            self.0.to_bytes()
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            UserPrincipal(Principal::from_bytes(bytes))
        }

        const BOUND: Bound = Principal::BOUND;
    }
}

impl super::UserPrimaryKey for UserPrincipal {
    fn generate() -> Self {
        UserPrincipal(ic_cdk::caller())
    }

    fn as_principal(&self) -> UserPrincipal {
        *self
    }
}
