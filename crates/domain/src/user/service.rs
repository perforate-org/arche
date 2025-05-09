use crate::{user::{
    entity::model::User,
    repository::{UserRepository, UserRepositoryError},
    value_object::{UserId, UserPrimaryKey},
}, PaperId};
use hexaurl::Error as HexaUrlError;
use thiserror::Error;

/// Errors that can occur during user service operations
#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("User not found")]
    NotFound,
    #[error("User with this ID already exists")]
    IdAlreadyExists,
    #[error("User with this primary key already exists")]
    PrimaryKeyAlreadyExists,
    #[error("Invalid user ID format: {0}")]
    InvalidId(HexaUrlError),
}

impl From<UserRepositoryError> for UserServiceError {
    fn from(error: UserRepositoryError) -> Self {
        match error {
            UserRepositoryError::NotFound => UserServiceError::NotFound,
            UserRepositoryError::IdAlreadyExists => UserServiceError::IdAlreadyExists,
            UserRepositoryError::PrimaryKeyAlreadyExists => UserServiceError::PrimaryKeyAlreadyExists,
        }
    }
}

impl From<HexaUrlError> for UserServiceError {
    fn from(error: HexaUrlError) -> Self {
        UserServiceError::InvalidId(error)
    }
}

/// Service for managing user operations
pub struct UserService<R>
where
    R: UserRepository,
{
    repository: R,
}

impl<R> UserService<R>
where
    R: UserRepository,
{
    /// Creates a new UserService with the provided repository
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    // Note: After future implementations of [Rust RFC 1210](https://rust-lang.github.io/rfcs/1210-impl-specialization.html), this method will generally be implemented using UserRepository::get, the current implementation will be specifically used for UserService<StableUserRepository, UserPrincipal>
    /// Retrieves users by their IDs
    ///
    /// # Arguments
    ///
    /// * `ids` - A vector of user IDs to search for
    ///
    /// # Returns
    ///
    /// A vector of optional users where Some(user) if found, None if not found
    pub fn get_users_by_id(&self, ids: &Vec<UserId>) -> Vec<Option<User>> {
        let mut users = Vec::new();
        let mut primary_keys = Vec::new();

        for id in ids {
            primary_keys.push(self.repository.get_primary_key(id));
        }

        for primary_key in primary_keys {
            if let Some(primary_key) = primary_key {
                users.push(self.repository.get_by_primary_key(&primary_key));
            } else {
                users.push(None);
            }
        }

        users
    }

    /// Retrieves users by their principals
    ///
    /// # Arguments
    ///
    /// * `principals` - A vector of user principals to search for
    ///
    /// # Returns
    ///
    /// A vector of optional users where Some(user) if found, None if not found
    pub fn get_users_by_primary_key(&self, primary_keys: &Vec<R::PrimaryKey>) -> Vec<Option<User>> {
        let mut users = Vec::new();
        for primary_key in primary_keys {
            if let Some(user) = self.repository.get_by_primary_key(primary_key) {
                users.push(Some(user));
            } else {
                users.push(None);
            }
        }
        users
    }

    /// Registers a new user
    ///
    /// # Arguments
    ///
    /// * `id` - The desired user ID
    /// * `name` - The user's display name
    ///
    /// # Returns
    ///
    /// * `Ok(User)` - The created user
    /// * `Err(UserServiceError)` - If registration fails
    pub fn register(
        &mut self,
    ) -> Result<User, UserServiceError> {
        let primary_key = R::PrimaryKey::generate();

        // Check if user already exists with this principal or id
        if self.repository.contains(&primary_key) {
            return Err(UserServiceError::PrimaryKeyAlreadyExists);
        }

        // Create new user
        let user = User::default();

        // Store in repository
        self.repository.add(primary_key, user.clone())?;

        Ok(user)
    }

    pub fn add_paper_as_lead_author(&mut self, primary_key: &R::PrimaryKey, paper_id: &PaperId) -> Result<(), UserServiceError> {
        let mut user = self.repository.get_by_primary_key(primary_key).ok_or(UserServiceError::NotFound)?;
        user.lead_authored_papers.push(*paper_id);
        self.repository.update(primary_key, user)?;
        Ok(())
    }

    pub fn remove_paper_as_lead_author(&mut self, primary_key: &R::PrimaryKey, paper_id: &PaperId) -> Result<(), UserServiceError> {
        let mut user = self.repository.get_by_primary_key(primary_key)
            .ok_or(UserServiceError::NotFound)?;

        // Find and remove the paper from lead_authored_papers
        let position = user.lead_authored_papers.iter().position(|id| id == paper_id);
        if let Some(index) = position {
            user.lead_authored_papers.remove(index);
            self.repository.update(primary_key, user)?;
            Ok(())
        } else {
            // Paper not found in user's authored papers
            Err(UserServiceError::NotFound)
        }
    }

    pub fn remove_paper_as_co_author(&mut self, primary_key: &R::PrimaryKey, paper_id: &PaperId) -> Result<(), UserServiceError> {
        let mut user = self.repository.get_by_primary_key(primary_key)
            .ok_or(UserServiceError::NotFound)?;

        // Find and remove the paper from co_authored_papers
        let position = user.co_authored_papers.iter().position(|id| id == paper_id);
        if let Some(index) = position {
            user.co_authored_papers.remove(index);
            self.repository.update(primary_key, user)?;
            Ok(())
        } else {
            // Paper not found in user's co-authored papers
            Err(UserServiceError::NotFound)
        }
    }
}
