use domain::user::{repository::UserRepository, UserPrimaryKey};
use std::marker::PhantomData;

mod query;
mod update;

pub struct UserController<R: UserRepository, K: UserPrimaryKey> {
    repository: R,
    _marker: PhantomData<K>,
}

impl<R: UserRepository, K: UserPrimaryKey> UserController<R, K> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            _marker: PhantomData,
        }
    }
}
