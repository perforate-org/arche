use crate::article::{
    entity::model::Article,
    repository::ArticleRepository,
    ArticleId, ArticleTitle, ArticleCategory,
};

pub struct ArticleService<R>
where
    R: ArticleRepository,
{
    repository: R,
}

impl<R> ArticleService<R>
where
    R: ArticleRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn get_articles_by_id(&self, ids: Vec<ArticleId>) -> Vec<Option<Article<R::UserPrimaryKey>>> {
        let mut articles = Vec::new();
        for id in ids {
            articles.push(self.repository.get(&id));
        }
        articles
    }

    pub fn create_draft(
        &mut self,
        lead_author: R::UserPrimaryKey,
        title: ArticleTitle,
        summary: String,
        content: String,
        categories: Vec<ArticleCategory>,
        tags: Vec<String>,
    ) -> ArticleId {
        let id = self.repository.generate_id();
        let article = Article::new_draft(lead_author, title, summary, content, categories, tags);
        self.repository.insert(id, article).unwrap();
        id
    }
}
