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
    pub fn fetch(&self, user_id: &str) -> Result<dto::User, String> {
        let user_id = UserId::new(user_id).map_err(|_| format!("Invalid user ID: {}", user_id))?;

        let user = match self.repository.get(&user_id) {
            Some(user) => user,
            None => return Err(format!("User not found: {}", user_id)),
        };

        Ok(user.into())
    }
}
