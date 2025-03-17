use crate::{
    article::{ArticleId, entity::model::Article},
    user,
};

/// Repository trait for read-only article operations.
/// Provides methods to retrieve articles and check for existence.
pub trait ArticleRepository {
    type UserPrimaryKey: user::UserPrimaryKey;

    /// Retrieves an article by its ID.
    ///
    /// # Arguments
    ///
    /// * `article_id` - The unique identifier of the article to retrieve
    ///
    /// # Returns
    ///
    /// * `Some(Article)` - The requested article if found
    /// * `None` - If the article doesn't exist
    fn get(&self, article_id: &ArticleId) -> Option<Article<Self::UserPrimaryKey>>;

    /// Checks if an article with the given ID exists in the repository.
    ///
    /// # Arguments
    ///
    /// * `article_id` - The unique identifier of the article to check
    ///
    /// # Returns
    ///
    /// * `true` - If the article exists
    /// * `false` - If the article doesn't exist
    fn contains(&self, article_id: &ArticleId) -> bool;

    /// Inserts or updates an article in the repository.
    ///
    /// # Arguments
    ///
    /// * `article_id` - The unique identifier for the article
    /// * `article` - The article entity to store
    ///
    /// # Returns
    ///
    /// * `Some(Article)` - The previous article with this ID, if it existed
    /// * `None` - If there was no previous article with this ID
    fn insert(&mut self, article_id: ArticleId, article: Article<Self::UserPrimaryKey>) -> Option<Article<Self::UserPrimaryKey>>;

    /// Removes an article from the repository.
    ///
    /// # Arguments
    ///
    /// * `article_id` - The unique identifier of the article to remove
    ///
    /// # Returns
    ///
    /// * `Some(Article)` - The removed article if found
    /// * `None` - If the article doesn't exist
    fn remove(&mut self, article_id: &ArticleId) -> Option<Article<Self::UserPrimaryKey>>;

    /// Generates a new article ID following the format `YYYY-MM-number`.
    ///
    /// Creates an identifier with the current year-month and the next available
    /// sequential number for the current month. The version is set to 1 (default)
    /// and not displayed in the string representation.
    ///
    /// # Returns
    ///
    /// * `ArticleId` - A new unique identifier with format `YYYY-MM-nnnn`
    ///   where nnnn is the next sequential number for the current month
    fn generate_id(&mut self) -> ArticleId;
}
