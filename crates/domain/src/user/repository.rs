use crate::user::{
    value_object::{UserId, UserPrimaryKey},
    entity::model::User,
};
use thiserror::Error;

/// Repository operation errors that may occur during user management
#[derive(Error, Debug)]
pub enum UserRepositoryError {
    #[error("User not found")]
    NotFound,
    #[error("User id already exists")]
    IdAlreadyExists,
    #[error("User primary key already exists")]
    PrimaryKeyAlreadyExists,
}

/// Repository trait for read-only User entity operations.
/// Provides methods to find users and check for existence without modifying state.
pub trait UserRepository {
    type PrimaryKey: UserPrimaryKey;

    /// Creates a new instance of the repository.
    ///
    /// This method should be implemented by concrete repository implementations
    /// to initialize a new repository instance with default settings.
    ///
    /// # Returns
    ///
    /// A new instance of the repository implementation.
    fn new() -> Self where Self: Sized;

    /// Gets a user by their unique ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to search for
    ///
    /// # Returns
    ///
    /// * `Some(User)` if the user was found
    /// * `None` if no user with the given ID exists
    fn get(&self, user_id: &UserId) -> Option<User>;

    /// Gets a user by their primary key.
    ///
    /// # Arguments
    ///
    /// * `primary_key` - The user primary key to search for
    ///
    /// # Returns
    ///
    /// * `Some(User)` if the user was found
    /// * `None` if no user with the given primary key exists
    fn get_by_primary_key(&self, primary_key: &Self::PrimaryKey) -> Option<User>;

    /// Checks if a user with the given primary key is contained.
    ///
    /// # Arguments
    ///
    /// * `primary_key` - The user primary key to check
    ///
    /// # Returns
    ///
    /// * `true` if a user with the given primary key exists
    /// * `false` if no user with the given primary key exists
    fn contains(&self, primary_key: &Self::PrimaryKey) -> bool;

    /// Checks if a user with the given ID is contained.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to check
    ///
    /// # Returns
    ///
    /// * `true` if a user with the given ID exists
    /// * `false` if no user with the given ID exists
    fn contains_id(&self, user_id: &UserId) -> bool;

    /// Retrieves the primary key of a user with the given ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to retrieve the primary key for
    ///
    /// # Returns
    ///
    /// * `Some(primary_key)` if a user with the given ID exists
    /// * `None` if no user with the given ID exists
    fn get_primary_key(&self, user_id: &UserId) -> Option<Self::PrimaryKey>;

    /// Retrieves the user ID of a user with the given primary key.
    ///
    /// # Arguments
    ///
    /// * `primary_key` - The user primary key to retrieve the ID for
    ///
    /// # Returns
    ///
    /// * `Some(user_id)` if a user with the given primary key exists
    /// * `None` if no user with the given primary key exists
    fn get_user_id(&self, primary_key: &Self::PrimaryKey) -> Option<UserId>;

    /// Adds a new user to the repository.
    ///
    /// # Arguments
    ///
    /// * `primary_key` - The user's primary_key identifier
    /// * `id` - The user's unique ID
    /// * `user` - The user entity to store
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the user was stored successfully
    /// * `Err(UserRepositoryError::IdAlreadyExists)` if the user ID already exists
    /// * `Err(UserRepositoryError::PrimaryKeyAlreadyExists)` if the user primary_key already exists
    fn add(&mut self, primary_key: Self::PrimaryKey, user: User) -> Result<(), UserRepositoryError>;

    /// Updates an existing user in the repository.
    ///
    /// # Arguments
    ///
    /// * `primary_key` - The user's primary_key identifier
    /// * `user` - The updated user entity
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the user was updated successfully
    /// * `Err(UserRepositoryError::NotFound)` if no user with the given primary_key exists
    fn update(&mut self, primary_key: &Self::PrimaryKey, user: User) -> Result<(), UserRepositoryError>;

    /// Updates a user's ID.
    ///
    /// # Arguments
    ///
    /// * `primary_key` - The user's primary_key identifier
    /// * `new_id` - The new ID to assign to the user
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the user's ID was updated successfully
    /// * `Err(UserRepositoryError::NotFound)` if no user with the old ID exists
    /// * `Err(UserRepositoryError::IdAlreadyExists)` if the new ID is already in use
    fn update_id(&mut self, primary_key: &Self::PrimaryKey, new_id: Option<UserId>) -> Result<(), UserRepositoryError>;

    /// Removes a user by their primary_key identifier.
    ///
    /// # Arguments
    ///
    /// * `primary_key` - The user primary_key to remove
    ///
    /// # Returns
    ///
    /// * `Ok(User)` with the removed user if successful
    /// * `Err(UserRepositoryError::NotFound)` if no user with the given primary_key exists
    fn remove(&mut self, primary_key: &Self::PrimaryKey) -> Result<User, UserRepositoryError>;
}
