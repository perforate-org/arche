use domain::{user::{repository::UserRepository, UserPrimaryKey}, paper::repository::PaperRepository};
use std::marker::PhantomData;

mod query;
mod update;

pub struct UserController<R: UserRepository, P: PaperRepository, K: UserPrimaryKey> {
    repository: R,
    paper_repository: P,
    _marker: PhantomData<K>,
}

impl<R: UserRepository, P: PaperRepository, K: UserPrimaryKey> UserController<R, P, K> {
    pub fn new(repository: R, paper_repository: P) -> Self {
        Self {
            repository,
            paper_repository,
            _marker: PhantomData,
        }
    }
}
