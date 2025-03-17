use candid::CandidType;
use serde::{Deserialize, Serialize};

/// Category for technical articles
#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ArticleCategory {
    Programming,
    SystemDesign,
    DevOps,
    Security,
    MachineLearning,
    Blockchain,
    Other(String),
}
