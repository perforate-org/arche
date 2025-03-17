use domain::user::{
    entity::dto,
    repository::UserRepository,
    UserId, UserPrimaryKey
};
use super::UserController;
use interface::user::*;

impl<R, K> UserController<R, K>
where
    R: UserRepository + Clone,
    K: UserPrimaryKey,
{
    pub fn fetch(&self, user_id: &UserId) -> Result<dto::User, String> {
        let user = match self.repository.get(user_id) {
            Some(user) => user,
            None => return Err(format!("User not found: {}", user_id)),
        };

        Ok(user.into())
    }

    pub fn fetch_with_str(
        &self,
        user_id: &String,
    ) -> Result<dto::User, String> {
        // Convert the user_id string to an UserId.
        let user_id = UserId::new(user_id).map_err(|_| format!("Invalid user ID: {}", user_id))?;

        // Call the get_user_profile function with the converted UserId.
        self.fetch(&user_id)
    }
}
