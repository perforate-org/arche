use ic_cdk::api::caller;
use candid::Principal;
use crate::infrastructure::user::repository::{StableUserRepository, UserRepository};

pub fn caller_is_not_anonymous() -> Result<(), String> {
    match caller() != Principal::anonymous() {
        true => Ok(()),
        false => Err("Caller is anonymous".to_string()),
    }
}

pub fn caller_is_user() -> Result<(), String> {
    let repository = StableUserRepository::new();
    match repository.contains(&caller().into()) {
        true => Ok(()),
        false => Err("Caller is not a user".to_string()),
    }
}
