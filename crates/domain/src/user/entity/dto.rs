use crate::{
    article::ArticleId,
    user::UserName,
};
use candid::{CandidType, Deserialize};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct User {
    pub name: UserName,
    pub lead_authored_articles: Vec<ArticleId>,
    pub co_authored_articles: Vec<ArticleId>,
}

#[cfg(feature = "entity")]
impl From<super::model::User> for User {
    fn from(user: super::model::User) -> Self {
        User {
            name: user.name,
            lead_authored_articles: user.lead_authored_articles,
            co_authored_articles: user.co_authored_articles,
        }
    }
}

#[cfg(feature = "entity")]
impl From<User> for super::model::User {
    fn from(user: User) -> Self {
        super::model::User {
            name: user.name,
            lead_authored_articles: user.lead_authored_articles,
            co_authored_articles: user.co_authored_articles,
        }
    }
}
