use std::str::FromStr;

use crate::{
    paper::{entity::dto::PaperIdTitle, repository::PaperRepository, PaperId},
    user::UserName, UserId,
};
use candid::{CandidType, Deserialize};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub name: UserName,
    pub lead_authored_papers: Vec<PaperIdTitle>,
    pub co_authored_papers: Vec<PaperIdTitle>,
}

#[cfg(feature = "entity")]
impl User {
    pub fn from_model(user: super::model::User, paper_repository: &impl PaperRepository) -> Self {
        let lead_authored_papers = user.lead_authored_papers.into_iter().filter_map(|paper| {
            let title = paper_repository.get_title(&paper)?.to_string();
            Some(PaperIdTitle { id: paper.to_string(), title })
        }).collect();
        let co_authored_papers = user.co_authored_papers.into_iter().filter_map(|paper| {
                    let title = paper_repository.get_title(&paper)?.to_string();
                    Some(PaperIdTitle { id: paper.to_string(), title })
                }).collect();

        User {
            id: user.id.map(|id| id.to_string()),
            name: user.name,
            lead_authored_papers,
            co_authored_papers,
        }
    }
}

#[cfg(feature = "entity")]
impl From<User> for super::model::User {
    fn from(user: User) -> Self {
        super::model::User {
            id: user.id.and_then(|id| UserId::new(&id).ok()),
            name: user.name,
            lead_authored_papers: user.lead_authored_papers
                .into_iter()
                .filter_map(|paper| PaperId::from_str(&paper.id).ok())
                .collect(),
            co_authored_papers: user.co_authored_papers
                .into_iter()
                .filter_map(|paper| PaperId::from_str(&paper.id).ok())
                .collect(),
        }
    }
}
