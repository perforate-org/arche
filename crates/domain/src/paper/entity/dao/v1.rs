use crate::{paper::{PaperCategory, PaperStatus, PaperTitle, Citation, PaperContents}, user::UserPrimaryKey};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
pub struct V1<K: UserPrimaryKey> {
    /// The lead author of the paper
    pub lead_author: K,
    /// Co-authors of the paper, if any
    pub co_authors: Vec<K>,
    /// Title of the paper
    pub title: PaperTitle,
    /// Abstract of the paper
    pub ab: String,
    /// Main content of the paper
    pub content: PaperContents,
    /// Categories this paper belongs to
    pub categories: Vec<PaperCategory>,
    /// Tags for better searchability
    pub tags: Vec<String>,
    /// Current status of the paper
    pub status: PaperStatus,
    /// When the paper was created
    pub created_at: u64,
    /// When the paper was last updated in nanoseconds since epoch
    pub updated_at: u64,
    /// Optional URL to cover image in
    pub cover_image: Option<String>,
    /// Contains identifiers of papers that are referenced by this paper, enabling meaningful cross-linking to related content.
    pub references: Vec<Citation>,
    /// List of identifiers for all papers that have cited this paper.
    pub citations: Vec<Citation>,
}
