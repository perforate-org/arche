use domain::{paper::repository::PaperRepository, user::{repository::UserRepository, UserPrimaryKey}};
use super::PaperController;
use interface::paper::*;

impl<A, U, UK> PaperController<A, U, UK>
where
    A: PaperRepository,
    U: UserRepository,
    UK: UserPrimaryKey,
{}
