use crate::{
    article::ArticleId,
    user::UserName,
};
use super::dao::{UserDao, UserDaoVersion, V1};

#[derive(Debug, Clone)]
pub struct User {
    pub name: UserName,
    pub lead_authored_articles: Vec<ArticleId>,
    pub co_authored_articles: Vec<ArticleId>,
}

impl User {
    pub fn new(name: UserName) -> Self {
        User {
            name,
            lead_authored_articles: Vec::new(),
            co_authored_articles: Vec::new(),
        }
    }
}

impl From<UserDao> for User {
    fn from(user_dao: UserDao) -> Self {
        match user_dao.version {
            UserDaoVersion::V1(v1) => User {
                name: v1.name,
                lead_authored_articles: v1.lead_authored_articles,
                co_authored_articles: v1.co_authored_articles,
            },
        }
    }
}

impl From<User> for UserDao {
    fn from(user: User) -> Self {
        UserDao {
            version: UserDaoVersion::V1(V1 {
                name: user.name,
                lead_authored_articles: user.lead_authored_articles,
                co_authored_articles: user.co_authored_articles,
            }),
        }
    }
}

impl User {
    pub fn add_lead_authored_article(&mut self, article_id: ArticleId) {
        self.lead_authored_articles.push(article_id);
    }

    pub fn add_co_authored_article(&mut self, article_id: ArticleId) {
        self.co_authored_articles.push(article_id);
    }
}
