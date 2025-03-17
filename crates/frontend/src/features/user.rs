use domain::user::{UserId, entity::dto::User};
use crate::context::service::{Service, Backend};

pub trait UserService {
    async fn fetch_user(&mut self, id: &UserId) -> Result<User, String>;
}

impl UserService for Service<Backend> {
    async fn fetch_user(&mut self, id: &UserId) -> Result<User, String> {
        self.query("fetch_user", id).await
    }
}
