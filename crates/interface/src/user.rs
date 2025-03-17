use crate::{CandidType, Response};
use domain::user::{UserName, entity::dto::User};
use serde::{Deserialize, Serialize};

pub mod register_user;

#[derive(Response, CandidType, Deserialize, Debug)]
pub struct UserResponse {
    pub user: User,
}
