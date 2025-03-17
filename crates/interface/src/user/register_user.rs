use crate::{CandidType, Request};
use domain::user::{UserName, UserId};
use serde::{Deserialize, Serialize};

#[derive(Request, CandidType, Clone, Serialize, Deserialize, Debug)]
pub struct RegisterUserRequest {
    pub id: UserId,
    pub name: UserName,
}
