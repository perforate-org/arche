use domain::{paper::repository::PaperRepository, user::{repository::UserRepository, UserPrimaryKey}};
use std::marker::PhantomData;

mod query;
mod update;

pub struct PaperController<A: PaperRepository, U: UserRepository, UK: UserPrimaryKey> {
    repository: A,
    user_repository: U,
    _marker: PhantomData<UK>,
}

impl<R: PaperRepository, U: UserRepository, UK: UserPrimaryKey> PaperController<R, U, UK> {
    pub fn new(repository: R, user_repository: U) -> Self {
        Self { repository, user_repository, _marker: PhantomData }
    }
}
