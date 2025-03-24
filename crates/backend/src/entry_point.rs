#![allow(unused)]

use domain::{
    paper::{
        entity::dto::Paper,
        value_object::*,
    },
    user::{
        entity::dto::User,
        value_object::*,
    },
};
use interface::user::*;
use interface::paper::*;

mod guards;
mod paper;
mod user;

ic_cdk::export_candid!();
