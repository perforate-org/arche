use domain::article::{ArticleId, entity::dto::Article};
use crate::context::service::{Service, Backend};

pub trait ArticleService {
    async fn fetch_article(&mut self, id: &ArticleId) -> Result<Article, String>;
}

impl ArticleService for Service<Backend> {
    async fn fetch_article(&mut self, id: &ArticleId) -> Result<Article, String> {
        self.query("fetch_article", id).await
    }
}
