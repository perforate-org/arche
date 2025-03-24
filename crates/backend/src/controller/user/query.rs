use domain::user::{
    entity::dto,
    repository::UserRepository,
    UserId, UserPrimaryKey, UserPrincipal
};
use super::UserController;
use interface::user::*;
use ic_cdk::api::caller;
use std::str::FromStr;

impl<R, K> UserController<R, K>
where
    R: UserRepository<PrimaryKey = UserPrincipal> + Clone,
    K: UserPrimaryKey,
{
    pub fn fetch(&self, user_id: &str) -> Result<dto::User, String> {
        if let Some(principal_str) = user_id.strip_prefix("p_") {
            let principal = UserPrincipal::from_str(principal_str)
                .map_err(|_| format!("Invalid principal: {}", principal_str))?;

            let user = match self.repository.get_by_primary_key(&principal) {
                Some(user) => user,
                None => return Err(format!("User not found by principal: {}", principal_str)),
            };

            Ok(user.into())
        } else {
            let user_id = UserId::new(user_id).map_err(|_| format!("Invalid user ID: {}", user_id))?;

            let user = match self.repository.get(&user_id) {
                Some(user) => user,
                None => return Err(format!("User not found: {}", user_id)),
            };

            Ok(user.into())
        }
    }

    pub fn fetch_caller(&self) -> Result<dto::User, String> {
        let caller = caller();

        let user = match self.repository.get_by_primary_key(&caller.into()) {
            Some(user) => user,
            None => return Err(format!("User not found: {}", caller)),
        };

        Ok(user.into())
    }

    pub fn is_registered(&self) -> bool {
        let caller = caller();

        self.repository.contains(&caller.into())
    }


    pub fn user_exists_by_id(&self, user_id: &str) -> Result<bool, String> {
        let user_id = UserId::new(user_id).map_err(|_| format!("Invalid user ID: {}", user_id))?;

        Ok(self.repository.contains_id(&user_id))
    }
}
