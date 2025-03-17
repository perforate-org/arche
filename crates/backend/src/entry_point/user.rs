use crate::{
    controller::user::UserController,
    infrastructure::user::repository::StableUserRepository,
};
use domain::user::{
    entity::dto::User,
    value_object::{UserId, UserPrincipal},
};
use ic_cdk_macros::*;
use interface::user::*;

fn controller() -> UserController<StableUserRepository, UserPrincipal> {
    UserController::<StableUserRepository, UserPrincipal>::new(
        StableUserRepository::new()
    )
}

#[query]
fn fetch_user(user_id: UserId) -> Result<User, String> {
    let controller = controller();

    controller.fetch(&user_id)
}

#[query]
fn fetch_user_with_str(user_id: String) -> Result<User, String> {
    let controller = controller();

    controller.fetch_with_str(&user_id)
}

#[update]
fn register_user(req: register_user::RegisterUserRequest) -> Result<(), String> {
    let mut controller = controller();

    controller.register(req)
}
