use candid::CandidType;
use serde::{Deserialize, Serialize};

/// Category for technical papers
#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum PaperCategory {
    Programming,
    SystemDesign,
    DevOps,
    Security,
    MachineLearning,
    Blockchain,
    Other(String),
}
