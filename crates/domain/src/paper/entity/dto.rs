use crate::{paper::{PaperCategory, PaperStatus, PaperTitle, Citation}, user::{UserId, UserName}};
use serde::Deserialize;
#[cfg(feature = "entity")]
use crate::{
    paper::entity::model,
    user::repository::UserRepository,
};

#[derive(Clone, Debug, candid::CandidType, Deserialize)]
pub struct Paper {
    /// The lead author of the paper
    pub lead_author: (String, UserName),
    /// Co-authors of the paper, if any
    pub co_authors: Vec<(String, UserName)>,
    /// Title of the paper
    pub title: PaperTitle,
    /// Abstract of the paper
    pub ab: String,
    /// Main content of the paper in Typst format
    pub content: String,
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

#[cfg(feature = "entity")]
impl Paper {
    pub fn from_model<T: UserRepository>(paper: model::Paper<T::PrimaryKey>, user_repo: &T) -> Option<Self> {
        let lead_author = match user_repo.get_user_id(&paper.lead_author) {
            Some(id) => (id.to_string(), user_repo.get_by_primary_key(&paper.lead_author)?.name),
            None => (format!("p_{}", paper.lead_author), user_repo.get_by_primary_key(&paper.lead_author)?.name),
        };
        let co_authors = paper.co_authors.into_iter().map(|key| -> Option<_> {
            let id = match user_repo.get_user_id(&key) {
                Some(id) => id.to_string(),
                None => format!("p_{}", key),
            };
            let user = user_repo.get_by_primary_key(&key)?;
            Some((id, user.name))
        }).collect::<Option<Vec<_>>>()?;

        Some(Paper {
            lead_author,
            co_authors,
            title: paper.title,
            ab: paper.ab,
            content: paper.content,
            categories: paper.categories,
            tags: paper.tags,
            status: paper.status,
            created_at: paper.created_at,
            updated_at: paper.updated_at,
            cover_image: paper.cover_image,
            references: paper.references,
            citations: paper.citations,
        })
    }

    pub fn into_model<T: UserRepository>(&self, user_repo: &T) -> Option<model::Paper<T::PrimaryKey>> {
        let lead_author = if let Some(primary_key_str) = self.lead_author.0.strip_prefix("p_") {
            primary_key_str.parse().ok()?
        } else {
            user_repo.get_primary_key(&UserId::new(&self.lead_author.0).ok()?)?
        };
        let co_authors = self
            .co_authors
            .iter()
            .map(|author| {
                if let Some(primary_key_str) = author.0.strip_prefix("p_") {
                    primary_key_str.parse().ok()
                } else {
                    user_repo.get_primary_key(&UserId::new(&author.0).ok()?)
                }
            })
            .collect::<Option<Vec<_>>>()?;

        Some(model::Paper {
            lead_author,
            co_authors,
            title: self.title.clone(),
            ab: self.ab.clone(),
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
