use crate::{
    paper::{PaperId, PaperSummary, PaperTitle, entity::model::Paper},
    user,
};

/// Repository trait for paper operations.
/// Provides methods to retrieve papers and check for existence.
pub trait PaperRepository {
    type UserPrimaryKey: user::UserPrimaryKey;

    /// Retrieves an paper by its ID.
    ///
    /// # Arguments
    ///
    /// * `paper_id` - The unique identifier of the paper to retrieve
    ///
    /// # Returns
    ///
    /// * `Some(Paper)` - The requested paper if found
    /// * `None` - If the paper doesn't exist
    fn get(&self, paper_id: &PaperId) -> Option<Paper<Self::UserPrimaryKey>>;

    /// Retrieves a summary of an paper by its ID.
    ///
    /// # Arguments
    ///
    /// * `paper_id` - The unique identifier of the paper to retrieve
    ///
    /// # Returns
    ///
    /// * `Some(PaperSummary)` - The requested paper summary if found
    /// * `None` - If the paper doesn't exist
    fn get_summary(&self, paper_id: &PaperId) -> Option<PaperSummary<Self::UserPrimaryKey>>;

    /// Retrieves the title of an paper by its ID.
    ///
    /// # Arguments
    ///
    /// * `paper_id` - The unique identifier of the paper to retrieve
    ///
    /// # Returns
    ///
    /// * `Some(PaperTitle)` - The requested paper title if found
    /// * `None` - If the paper doesn't exist
    fn get_title(&self, paper_id: &PaperId) -> Option<PaperTitle>;

    /// Checks if an paper with the given ID exists in the repository.
    ///
    /// # Arguments
    ///
    /// * `paper_id` - The unique identifier of the paper to check
    ///
    /// # Returns
    ///
    /// * `true` - If the paper exists
    /// * `false` - If the paper doesn't exist
    fn contains(&self, paper_id: &PaperId) -> bool;

    /// Iterates over all paper summaries in the repository.
    ///
    /// # Returns
    ///
    /// An iterator over all paper summaries in the repository.
    fn iter_summary(&self) -> impl Iterator<Item = PaperSummary<Self::UserPrimaryKey>>;

    /// Inserts or updates an paper in the repository.
    ///
    /// # Arguments
    ///
    /// * `paper_id` - The unique identifier for the paper
    /// * `paper` - The paper entity to store
    ///
    /// # Returns
    ///
    /// * `Some(Paper)` - The previous paper with this ID, if it existed
    /// * `None` - If there was no previous paper with this ID
    fn insert(&mut self, paper_id: PaperId, paper: Paper<Self::UserPrimaryKey>) -> Option<Paper<Self::UserPrimaryKey>>;

    /// Removes an paper from the repository.
    ///
    /// # Arguments
    ///
    /// * `paper_id` - The unique identifier of the paper to remove
    ///
    /// # Returns
    ///
    /// * `Some(Paper)` - The removed paper if found
    /// * `None` - If the paper doesn't exist
    fn remove(&mut self, paper_id: &PaperId) -> Option<Paper<Self::UserPrimaryKey>>;

    /// Generates a new paper ID following the format `YYYY-MM-number`.
    ///
    /// Creates an identifier with the current year-month and the next available
    /// sequential number for the current month. The version is set to 1 (default)
    /// and not displayed in the string representation.
    ///
    /// # Returns
    ///
    /// * `PaperId` - A new unique identifier with format `YYYY-MM-nnnn`
    ///   where nnnn is the next sequential number for the current month
    fn generate_id(&mut self) -> PaperId;
}
