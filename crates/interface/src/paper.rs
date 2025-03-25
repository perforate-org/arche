use crate::{Request, Response, CandidType, Deserialize, Serialize};
use domain::{
    paper::{entity::dto::Paper, value_object::{PaperCategory, PaperStatus}},
    PaperId, UserId,
};

/// Status of an paper
#[derive(CandidType, Clone, Serialize, Deserialize, Response, Debug, PartialEq)]
pub struct PaperSummaryDto {
    pub id: String,
    pub title: String,
    pub lead_author_id: String,
    pub lead_author_name: String,
}
