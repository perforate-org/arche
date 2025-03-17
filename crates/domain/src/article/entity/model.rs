use crate::{article::{ArticleCategory, ArticleStatus, ArticleTitle, Citation}, user::UserPrimaryKey};
use super::dao::{ArticleDao, ArticleDaoVersion, V1};
use util::time::now;

#[derive(Clone, Debug)]
pub struct Article<K: UserPrimaryKey> {
    /// The lead author of the article
    pub lead_author: K,
    /// Co-authors of the article, if any
    pub co_authors: Vec<K>,
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

impl<K: UserPrimaryKey> Article<K> {
    /// Creates a new draft article
    pub fn new_draft(
        lead_author: K,
        title: ArticleTitle,
        summary: String,
        content: String,
        categories: Vec<ArticleCategory>,
        tags: Vec<String>,
    ) -> Self {
        let now = now();

        Self {
            lead_author,
            co_authors: Vec::new(),
            title,
            summary,
            content,
            categories,
            tags,
            status: ArticleStatus::Draft,
            created_at: now,
            updated_at: now,
            cover_image: None,
            references: Vec::new(),
            citations: Vec::new(),
        }
    }

    /// Publishes a draft article
    pub fn publish(&mut self) -> Result<(), String> {
        if self.status != ArticleStatus::Draft {
            return Err("Only draft articles can be published".to_string());
        }
        self.status = ArticleStatus::Published;
        self.updated_at = now();
        Ok(())
    }

    pub fn unpublish(&mut self) -> Result<(), String> {
        if self.status != ArticleStatus::Published {
            return Err("Only published articles can be unpublished".to_string());
        }
        self.status = ArticleStatus::Draft;
        self.updated_at = now();
        Ok(())
    }

    pub fn is_author(&self, user: &K) -> bool {
        self.lead_author == *user || self.co_authors.contains(user)
    }
}

impl<K: UserPrimaryKey> From<ArticleDao<K>> for Article<K> {
    fn from(article_dao: ArticleDao<K>) -> Self {
        match article_dao.version {
            ArticleDaoVersion::V1(v1) => Article {
                lead_author: v1.lead_author,
                co_authors: v1.co_authors,
                title: v1.title,
                summary: v1.summary,
                content: v1.content,
                categories: v1.categories,
                tags: v1.tags,
                status: v1.status,
                created_at: v1.created_at,
                updated_at: v1.updated_at,
                cover_image: v1.cover_image,
                references: v1.references,
                citations: v1.citations,
            },
        }
    }
}

impl<K: UserPrimaryKey> From<Article<K>> for ArticleDao<K> {
    fn from(article: Article<K>) -> Self {
        ArticleDao {
            version: ArticleDaoVersion::V1(V1 {
                lead_author: article.lead_author,
                co_authors: article.co_authors,
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
    }
}
