use crate::{article::{ArticleCategory, ArticleStatus, ArticleTitle, Citation}, user::{UserId, UserName}};
use serde::Deserialize;
#[cfg(feature = "entity")]
use crate::{
    article::entity::model,
    user::repository::UserRepository,
};

#[derive(Clone, Debug, candid::CandidType, Deserialize)]
pub struct Article {
    /// The lead author of the article
    pub lead_author: (String, UserName),
    /// Co-authors of the article, if any
    pub co_authors: Vec<(String, UserName)>,
    /// Title of the article
    pub title: ArticleTitle,
    /// Brief summary of the article
    pub summary: String,
    /// Main content of the article in Typst format
    pub content: String,
    /// Categories this article belongs to
    pub categories: Vec<ArticleCategory>,
    /// Tags for better searchability
    pub tags: Vec<String>,
    /// Current status of the article
    pub status: ArticleStatus,
    /// When the article was created
    pub created_at: u64,
    /// When the article was last updated in nanoseconds since epoch
    pub updated_at: u64,
    /// Optional URL to cover image in
    pub cover_image: Option<String>,
    /// Contains identifiers of articles that are referenced by this article, enabling meaningful cross-linking to related content.
    pub references: Vec<Citation>,
    /// List of identifiers for all articles that have cited this article.
    pub citations: Vec<Citation>,
}

#[cfg(feature = "entity")]
impl Article {
    pub fn from_model<T: UserRepository>(article: model::Article<T::PrimaryKey>, user_repo: &T) -> Option<Self> {
        let lead_author = (user_repo.get_user_id(&article.lead_author)?.to_string(), user_repo.get_by_primary_key(&article.lead_author)?.name);
        let co_authors = article.co_authors.into_iter().map(|key| -> Option<_> {
            let id = user_repo.get_user_id(&key)?;
            let user = user_repo.get_by_primary_key(&key)?;
            Some((id.to_string(), user.name))
        }).collect::<Option<Vec<_>>>()?;

        Some(Article {
            lead_author,
            co_authors,
            title: article.title,
            summary: article.summary,
            content: article.content,
            categories: article.categories,
            tags: article.tags,
            status: article.status,
            created_at: article.created_at,
            updated_at: article.updated_at,
            cover_image: article.cover_image,
            references: article.references,
            citations: article.citations,
        })
    }

    pub fn into_model<T: UserRepository>(&self, user_repo: &T) -> Option<model::Article<T::PrimaryKey>> {
        let lead_author = user_repo.get_primary_key(&UserId::new(&self.lead_author.0).ok()?)?;
        let co_authors = self
            .co_authors
            .iter()
            .map(|author| user_repo.get_primary_key(&UserId::new(&author.0).ok()?))
            .collect::<Option<Vec<_>>>()?;

        Some(model::Article {
            lead_author,
            co_authors,
            title: self.title.clone(),
            summary: self.summary.clone(),
            content: self.content.clone(),
            categories: self.categories.clone(),
            tags: self.tags.clone(),
            status: self.status.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            cover_image: self.cover_image.clone(),
            references: self.references.clone(),
            citations: self.citations.clone(),
        })
    }
}
