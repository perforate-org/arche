use crate::{article::{ArticleCategory, ArticleStatus, ArticleTitle, Citation}, user::UserPrimaryKey};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
pub struct V1<K: UserPrimaryKey> {
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
