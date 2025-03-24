use candid::CandidType;
use serde::{Deserialize, Serialize};

/// Status of an paper
#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum PaperStatus {
    /// Draft paper, only visible to authors
    Draft,
    /// Published paper, visible to all
    Published,
    /// Paper under review
    UnderReview,
    /// Paper archived/no longer active
    Archived,
}
