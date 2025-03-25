use domain::{
    user::{
        UserPrimaryKey, UserPrincipal,
        repository::UserRepository,
        service::UserService,
        entity::dto
    },
    paper::repository::PaperRepository
};
use super::UserController;
use interface::user::*;
use ic_cdk::api::{caller, print};

impl<R, P, K> UserController<R, P, K>
where
    R: UserRepository<PrimaryKey = UserPrincipal> + Clone,
    P: PaperRepository + Clone,
    K: UserPrimaryKey,
{
    pub fn register(&mut self) -> Result<(), String> {
        let mut user_service: UserService<R> = UserService::new(self.repository.clone());

        user_service.register().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn update_caller(&mut self, user: dto::User) -> Result<(), String> {
        let mut user_repository = self.repository.clone();

        user_repository.update(&caller().into(), user.into()).map_err(|e| {
            print(format!("Failed to update user: {}", e));
            e.to_string()
        })?;

        Ok(())
    }
}
