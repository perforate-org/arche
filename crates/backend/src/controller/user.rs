use crate::use_case::user::UserUseCase;
use domain::{
    user::{
        entity::dto,
        repository::UserRepository,
        UserPrimaryKey,
        UserPrincipal
    },
    paper::repository::PaperRepository
};
use ic_cdk::api::caller;

pub struct UserController<R: UserRepository, P: PaperRepository, K: UserPrimaryKey> {
    use_case: UserUseCase<R, P, K>,
}

impl<R, P, K> UserController<R, P, K>
where
    R: UserRepository<PrimaryKey = UserPrincipal> + Clone,
    P: PaperRepository + Clone,
    K: UserPrimaryKey,
{
    pub fn new(repository: R, paper_repository: P) -> Self {
        Self {
            use_case: UserUseCase::new(repository, paper_repository),
        }
    }

    pub fn fetch(&self, user_id: &str) -> Result<dto::User, String> {
        let user = self.use_case.get_user(user_id)?;
        Ok(dto::User::from_model(user, self.use_case.get_paper_repository()))
    }

    pub fn fetch_caller(&self) -> Result<dto::User, String> {
        let caller_principal: UserPrincipal = caller().into();
        let user = self.use_case.get_user_by_principal(caller_principal)?;
        Ok(dto::User::from_model(user, self.use_case.get_paper_repository()))
    }

    pub fn is_registered(&self) -> bool {
        let caller_principal: UserPrincipal = caller().into();
        self.use_case.is_registered(&caller_principal)
    }

    pub fn user_exists_by_id(&self, user_id: &str) -> Result<bool, String> {
        self.use_case.user_exists_by_id(user_id)
    }

    pub fn register(&mut self) -> Result<(), String> {
        self.use_case.register().map(|_| ())
    }

    pub fn update_caller(&mut self, user_dto: dto::User) -> Result<(), String> {
        let caller_principal: UserPrincipal = caller().into();
        let user_model = user_dto.into();
        self.use_case.update_user(caller_principal, user_model)
    }
}
