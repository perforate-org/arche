mod id;
mod key;
mod title;
pub use id::ArticleId;
pub use key::ArticleKey;
pub use title::ArticleTitle;

use crate::{util::time::now, UserId};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

/// Status of an article
#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ArticleStatus {
    /// Draft article, only visible to authors
    Draft,
    /// Published article, visible to all
    Published,
    /// Article under review
    UnderReview,
    /// Article archived/no longer active
    Archived,
}

/// Category for technical articles
#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ArticleCategory {
    Programming,
    SystemDesign,
    DevOps,
    Security,
    MachineLearning,
    Blockchain,
    Other(String),
}

/// Represents a technical article in the system
#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
pub struct Article {
    /// The primary author of the article
    pub primary_author: UserId,
    /// Co-authors of the article, if any
    pub co_authors: Vec<UserId>,
    /// Title of the article
    pub title: ArticleTitle,
    /// Brief summary of the article
    pub summary: String,
    /// Main content of the article in Markdown format
    pub content: String,
    /// Categories this article belongs to
    pub categories: Vec<ArticleCategory>,
    /// Tags for better searchability
    pub tags: Vec<String>,
    /// Current status of the article
    pub status: ArticleStatus,
    /// When the article was created
    pub created_at: u64,
    /// When the article was last updated
    pub updated_at: u64,
    /// Number of views
    pub view_count: u64,
    /// Optional URL to cover image
    pub cover_image: Option<String>,
}

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for Article {
        const BOUND: Bound = Bound::Unbounded;
        fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
            Cow::Owned(candid::encode_one(self).unwrap())
        }

        fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
            candid::decode_one(&bytes).unwrap()
        }
    }
}

impl From<Principal> for UserId {
    fn from(principal: Principal) -> Self {
        UserId::new(&principal.to_string()).unwrap()
    }
}

impl Article {
    /// Creates a new draft article
    pub fn new_draft(
        primary_author: UserId,
        title: ArticleTitle,
        summary: String,
        content: String,
        categories: Vec<ArticleCategory>,
        tags: Vec<String>,
    ) -> Self {
        let now = now();

        Self {
            primary_author,
            co_authors: Vec::new(),
            title,
            summary,
            content,
            categories,
            tags,
            status: ArticleStatus::Draft,
            created_at: now,
            updated_at: now,
            view_count: 0,
            cover_image: None,
        }
    }

    /// Publishes a draft article
    pub fn publish(&mut self) -> Result<(), &'static str> {
        if self.status != ArticleStatus::Draft {
            return Err("Only draft articles can be published");
        }
        self.status = ArticleStatus::Published;
        self.updated_at = now();
        Ok(())
    }

    /// Increments the view count
    pub fn increment_views(&mut self) {
        self.view_count += 1;
    }
}
