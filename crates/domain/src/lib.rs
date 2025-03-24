pub mod paper;
pub mod user;

pub use paper::value_object::{id::*, title::*};
pub use user::value_object::{id::*, name::*};
#[cfg(feature = "canister")]
pub use user::value_object::principal::*;
