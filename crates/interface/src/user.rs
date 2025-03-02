use crate::{CandidType, Request, Response};
use common::UserId;
use serde::{Deserialize, Serialize};

#[derive(Response, CandidType, Serialize, Deserialize, Debug)]
pub struct UserProfileResponse {
    pub name: String,
}
