use crate::{
    controller::user::UserController,
    infrastructure::{user::repository::StableUserRepository, paper::repository::StablePaperRepository},
};
use domain::user::{
    entity::dto::User,
    UserPrincipal,
};
use ic_cdk_macros::*;
use interface::user::*;
use super::guards::{caller_is_user, caller_is_not_anonymous};

fn controller() -> UserController<StableUserRepository, StablePaperRepository, UserPrincipal> {
    UserController::<StableUserRepository, StablePaperRepository, UserPrincipal>::new(
        StableUserRepository::new(),
        StablePaperRepository::new(),
    )
}

#[query]
fn fetch_user(user_id: String) -> Result<User, String> {
    let controller = controller();

    controller.fetch(&user_id)
}

#[query(guard = "caller_is_not_anonymous")]
fn is_registered() -> bool {
    let controller = controller();

    controller.is_registered()
}

#[query(guard = "caller_is_not_anonymous")]
fn fetch_caller() -> Result<User, String> {
    let controller = controller();

    controller.fetch_caller()
}

#[query]
fn user_exists_by_id(user_id: String) -> Result<bool, String> {
    let controller = controller();

    controller.user_exists_by_id(&user_id)
}

#[update(guard = "caller_is_not_anonymous")]
fn register_user() -> Result<(), String> {
    let mut controller = controller();

    controller.register()
}

#[update(guard = "caller_is_user")]
fn update_caller(user: User) -> Result<(), String> {
    let mut controller = controller();

    controller.update_caller(user)
}
