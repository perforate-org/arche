use candid::CandidType;
use serde::{Deserialize, Serialize};

/// Status of an article
#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ArticleStatus {
    /// Draft article, only visible to authors
    Draft,
    /// Published article, visible to all
    Published,
    /// Article under review
    UnderReview,
    /// Article archived/no longer active
    Archived,
}
