use crate::{
    article::ArticleId,
    user::UserName,
};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct V1 {
    pub name: UserName,
    pub lead_authored_articles: Vec<ArticleId>,
    pub co_authored_articles: Vec<ArticleId>,
}
