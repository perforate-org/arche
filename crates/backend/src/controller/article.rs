use domain::{article::repository::ArticleRepository, user::{repository::UserRepository, UserPrimaryKey}};
use std::marker::PhantomData;

mod query;
mod update;

pub struct ArticleController<A: ArticleRepository, U: UserRepository, UK: UserPrimaryKey> {
    repository: A,
    user_repository: U,
    _marker: PhantomData<UK>,
}

impl<R: ArticleRepository, U: UserRepository, UK: UserPrimaryKey> ArticleController<R, U, UK> {
    pub fn new(repository: R, user_repository: U) -> Self {
        Self { repository, user_repository, _marker: PhantomData }
    }
}
