use crate::{paper::{repository::PaperRepository, Citation, PaperCategory, PaperContents, PaperId, PaperStatus, PaperTitle}, user::UserPrimaryKey};
use super::dao::{PaperDao, PaperDaoVersion, V1};
use util::time::now;

#[derive(Clone, Debug)]
pub struct Paper<K: UserPrimaryKey> {
    /// The unique identifier of the paper
    pub id: PaperId,
    /// The lead author of the paper
    pub lead_author: K,
    /// Co-authors of the paper, if any
    pub co_authors: Vec<K>,
    /// Title of the paper
    pub title: PaperTitle,
    /// Abstract of the paper
    pub ab: String,
    /// Main content of the paper in Typst format
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

impl<K: UserPrimaryKey> Paper<K> {
    /// Creates a new draft paper
    pub fn new_draft(
        lead_author: K,
        repository: &mut impl PaperRepository,
    ) -> Self {
        let now = now();
        let id = repository.generate_id();

        Self {
            id,
            lead_author,
            co_authors: Vec::new(),
            title: PaperTitle::new("New Paper").unwrap(),
            ab: String::new(),
            content: PaperContents::default(),
            categories: vec![],
            tags: vec![],
            status: PaperStatus::Draft,
            created_at: now,
            updated_at: now,
            cover_image: None,
            references: Vec::new(),
            citations: Vec::new(),
        }
    }

    /// Publishes a draft paper
    pub fn publish(&mut self) -> Result<(), String> {
        if self.status != PaperStatus::Draft {
            return Err("Only draft papers can be published".to_string());
        }
        self.status = PaperStatus::Published;
        self.updated_at = now();
        Ok(())
    }

    pub fn unpublish(&mut self) -> Result<(), String> {
        if self.status != PaperStatus::Published {
            return Err("Only published papers can be unpublished".to_string());
        }
        self.status = PaperStatus::Draft;
        self.updated_at = now();
        Ok(())
    }

    pub fn is_author(&self, user: &K) -> bool {
        self.lead_author == *user || self.co_authors.contains(user)
    }
}

impl<K: UserPrimaryKey> Paper<K> {
    pub fn from_dao(paper_dao: PaperDao<K>, id: PaperId) -> Self {
        match paper_dao.version {
            PaperDaoVersion::V1(v1) => Paper {
                id,
                lead_author: v1.lead_author,
                co_authors: v1.co_authors,
                title: v1.title,
                ab: v1.ab,
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

impl<K: UserPrimaryKey> From<Paper<K>> for PaperDao<K> {
    fn from(paper: Paper<K>) -> Self {
        PaperDao {
            version: PaperDaoVersion::V1(V1 {
                lead_author: paper.lead_author,
                co_authors: paper.co_authors,
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
    }
}
