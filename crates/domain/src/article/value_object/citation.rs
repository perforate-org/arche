use super::ArticleId;
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum Citation {
    Article(ArticleId),
    Url(String),
    Other(String),
}
