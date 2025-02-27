use crate::{Request, Response, CandidType};
use common::{UserId, PostId};
use serde::{Serialize, Deserialize};

#[derive(Request, Clone, CandidType, Debug)]
pub struct GetPostRequest {
    user: UserId,
    post: PostId,
}

#[derive(Response, CandidType, Serialize, Deserialize, Debug)]
pub struct GetPostResponse {
    user: UserId,
    post: PostId,
}
