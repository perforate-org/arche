use crate::{Request, Response, CandidType};
use common::{UserId, ArticleId};
use serde::{Serialize, Deserialize};

#[derive(Request, Clone, CandidType, Debug)]
pub struct GetArticleRequest {
    user: UserId,
    article: ArticleId,
}

#[derive(Response, CandidType, Serialize, Deserialize, Debug)]
pub struct GetArticleResponse {
    user: UserId,
    article: ArticleId,
}
