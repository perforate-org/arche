use domain::{article::{
    repository::ArticleRepository,
    entity::dto,
    ArticleId,
}, user::{repository::UserRepository, UserPrimaryKey}};
use super::ArticleController;
use std::str::FromStr;
use interface::article::*;

impl<A, U, UK> ArticleController<A, U, UK>
where
    A: ArticleRepository<UserPrimaryKey = UK> + Clone,
    U: UserRepository<PrimaryKey = UK> + Clone,
    UK: UserPrimaryKey,
{
    pub fn fetch(&self, article_id: &str) -> Result<dto::Article, String> {
        let article_id = ArticleId::from_str(article_id).map_err(|e| format!("Invalid article ID: {}", e))?;

        let article = match self.repository.get(&article_id) {
            Some(article) => article,
            None => return Err(format!("Article not found: {}", article_id)),
        };
        let res = dto::Article::from_model(article, &self.user_repository)
            .ok_or_else(|| format!("Failed to convert Article model to DTO for article: {}", article_id))?;
        Ok(res)
    }
}
