use crate::{
    paper::PaperId,
    user::UserName, UserId,
};
use candid::{CandidType, Deserialize};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub name: UserName,
    pub lead_authored_papers: Vec<PaperId>,
    pub co_authored_papers: Vec<PaperId>,
}

#[cfg(feature = "entity")]
impl From<super::model::User> for User {
    fn from(user: super::model::User) -> Self {
        User {
            id: user.id.map(|id| id.to_string()),
            name: user.name,
            lead_authored_papers: user.lead_authored_papers,
            co_authored_papers: user.co_authored_papers,
        }
    }
}

#[cfg(feature = "entity")]
impl From<User> for super::model::User {
    fn from(user: User) -> Self {
        super::model::User {
            id: user.id.and_then(|id| UserId::new(&id).ok()),
            name: user.name,
            lead_authored_papers: user.lead_authored_papers,
            co_authored_papers: user.co_authored_papers,
        }
    }
}
