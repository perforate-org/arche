use super::PaperId;
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum Citation {
    Paper(PaperId),
    Url(String),
    Other(String),
}
