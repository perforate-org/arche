use domain::{article::repository::ArticleRepository, user::{principal::UserPrincipal, repository::UserRepository, UserPrimaryKey}};
use super::ArticleController;
use domain::article::ArticleId;
use interface::article::*;

impl<A, U, UK> ArticleController<A, U, UK>
where
    A: ArticleRepository,
    U: UserRepository,
    UK: UserPrimaryKey,
{}
