use crate::{Request, Response, CandidType};
use common::UserId;
use serde::{Serialize, Deserialize};

#[derive(Response, CandidType, Serialize, Deserialize, Debug)]
pub struct UserProfileResponse {
    pub name: String,
}
