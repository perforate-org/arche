use domain::user::{
    UserPrimaryKey,
    repository::UserRepository,
    service::UserService,
};
use super::UserController;
use interface::user::*;

impl<R, K> UserController<R, K>
where
    R: UserRepository + Clone,
    K: UserPrimaryKey,
{
    pub fn register(&mut self, req: register_user::RegisterUserRequest) -> Result<(), String> {
        let mut user_service: UserService<R> = UserService::new(self.repository.clone());

        user_service.register(req.id, req.name).map_err(|e| e.to_string())?;
        Ok(())
    }
}
