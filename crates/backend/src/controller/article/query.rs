use domain::{article::{
    repository::ArticleRepository,
    entity::dto,
    ArticleId,
}, user::{principal::UserPrincipal, repository::UserRepository, UserPrimaryKey}};
use super::ArticleController;
use interface::article::*;

impl<A, U, UK> ArticleController<A, U, UK>
where
    A: ArticleRepository<UserPrimaryKey = UK> + Clone,
    U: UserRepository<PrimaryKey = UK> + Clone,
    UK: UserPrimaryKey,
{
    pub fn fetch(&self, article_id: &ArticleId) -> Result<dto::Article, String> {
        let article = match self.repository.get(article_id) {
            Some(article) => article,
            None => return Err(format!("Article not found: {}", article_id)),
        };
        let res = dto::Article::from_model(article, &self.user_repository)
            .ok_or_else(|| format!("Failed to convert Article model to DTO for article: {}", article_id))?;
        Ok(res)
    }
}
