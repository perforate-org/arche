mod key;
mod id;
mod title;
pub use key::PostKey;
pub use id::PostId;
pub use title::PostTitle;

use candid::CandidType;
use serde::{Serialize, Deserialize};
use crate::UserId;

/// Represents a post in the system.
#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
pub struct Post {
    /// The primary author of the post
    pub primary_author: UserId,
    /// Co-authors of the post, if any
    pub co_authors: Vec<UserId>,
    /// Title of the post
    pub title: PostTitle,
    /// Content of the post
    pub content: String,
    /// When the post was created
    pub created_at: u64,
    /// When the post was last updated
    pub updated_at: u64,
}
