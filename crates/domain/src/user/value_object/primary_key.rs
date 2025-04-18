use core::hash::Hash;
use std::{fmt::Display, str::FromStr};
#[cfg(feature = "canister")]
use super::UserPrincipal;

/// Represents a unique identifier for a user within the system.
///
/// This trait defines a system-wide primary key for user entities that is:
/// - Automatically generated by the application layer
/// - Guaranteed to be globally unique across the entire system
/// - Used for reliable identification and retrieval of user records
/// - Immutable once assigned to a user
pub trait UserPrimaryKey: Eq + PartialEq + Hash + Ord + PartialOrd + Clone + Copy + Sized + Send + Sync + Display + FromStr
{
    /// Generates a new user primary key.
    ///
    /// This method creates a unique identifier for a user that satisfies the system's
    /// requirements for uniqueness, persistence, and compatibility with the storage layer.
    /// Each implementation should ensure the generated key will not collide with existing keys.
    fn generate() -> Self;

    /// Converts this primary key to a UserPrincipal.
    ///
    /// This method is available only when the "canister" feature is enabled,
    /// allowing primary keys to be converted to the specialized principal format
    /// used by Internet Computer canisters.
    #[cfg(feature = "canister")]
    fn as_principal(&self) -> UserPrincipal;
}
