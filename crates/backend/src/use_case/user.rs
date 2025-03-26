use domain::{
    user::{
        entity::model::User,
        repository::UserRepository,
        service::UserService,
        UserId, UserPrimaryKey, UserPrincipal
    },
    paper::repository::PaperRepository,
};
use std::marker::PhantomData;
use std::str::FromStr;

pub struct UserUseCase<R: UserRepository, P: PaperRepository, K: UserPrimaryKey> {
    repository: R,
    paper_repository: P,
    _marker: PhantomData<K>,
}

impl<R, P, K> UserUseCase<R, P, K>
where
    R: UserRepository<PrimaryKey = UserPrincipal> + Clone,
    P: PaperRepository + Clone,
    K: UserPrimaryKey,
{
    pub fn new(repository: R, paper_repository: P) -> Self {
        Self {
            repository,
            paper_repository,
            _marker: PhantomData,
        }
    }

    // Get user model from user ID
    pub fn get_user(&self, user_id: &str) -> Result<User, String> {
        if let Some(principal_str) = user_id.strip_prefix("p_") {
            let principal = UserPrincipal::from_str(principal_str)
                .map_err(|_| format!("Invalid principal: {}", principal_str))?;

            self.repository.get_by_primary_key(&principal)
                .ok_or_else(|| format!("User not found by principal: {}", principal_str))
        } else {
            let user_id = UserId::new(user_id)
                .map_err(|_| format!("Invalid user ID: {}", user_id))?;

            self.repository.get(&user_id)
                .ok_or_else(|| format!("User not found: {}", user_id))
        }
    }

    // Retrieve user from principal
    pub fn get_user_by_principal(&self, principal: UserPrincipal) -> Result<User, String> {
        self.repository.get_by_primary_key(&principal)
            .ok_or_else(|| format!("User not found: {}", principal))
    }

    // User registration status check
    pub fn is_registered(&self, principal: &UserPrincipal) -> bool {
        self.repository.contains(principal)
    }

    // User ID existence check
    pub fn user_exists_by_id(&self, user_id: &str) -> Result<bool, String> {
        let user_id = UserId::new(user_id)
            .map_err(|_| format!("Invalid user ID: {}", user_id))?;
        Ok(self.repository.contains_id(&user_id))
    }

    // User Registration
    pub fn register(&mut self) -> Result<User, String> {
        let mut user_service: UserService<R> = UserService::new(self.repository.clone());
        user_service.register().map_err(|e| e.to_string())
    }

    // User Information Update
    pub fn update_user(&mut self, principal: UserPrincipal, user: User) -> Result<(), String> {
        let mut user_repository = self.repository.clone();
        user_repository.update(&principal, user)
            .map_err(|e| e.to_string())
    }

    // Get a reference to the paper repository
    pub fn get_paper_repository(&self) -> &P {
        &self.paper_repository
    }
}
