use domain::{
    article::{
        entity::dto::Article,
        value_object::*,
    },
    user::{
        entity::dto::User,
        value_object::*,
    },
};
use interface::user::*;
use interface::article::*;

mod article;
mod user;

ic_cdk::export_candid!();
