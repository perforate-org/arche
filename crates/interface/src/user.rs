use crate::{CandidType, Response};
use domain::user::entity::dto::User;
use serde::Deserialize;

pub mod register_user;

#[derive(Response, CandidType, Deserialize, Debug)]
pub struct UserResponse {
    pub user: User,
}
