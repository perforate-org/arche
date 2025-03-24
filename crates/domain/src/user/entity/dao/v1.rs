use crate::{
    paper::PaperId,
    user::UserName,
};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct V1 {
    pub name: UserName,
    pub lead_authored_papers: Vec<PaperId>,
    pub co_authored_papers: Vec<PaperId>,
}
