#![allow(unused)]

use crate::{
    controller::paper::PaperController,
    infrastructure::{
        paper::repository::StablePaperRepository,
        user::repository::StableUserRepository,
    },
};
use domain::{paper::entity::dto::Paper, UserPrincipal};
use ic_cdk::api::caller;
use ic_cdk_macros::*;
use interface::paper::*;
use super::guards::{caller_is_user, caller_is_not_anonymous};

fn controller() -> PaperController<StablePaperRepository, StableUserRepository, UserPrincipal> {
    PaperController::<StablePaperRepository, StableUserRepository, UserPrincipal>::new(
        StablePaperRepository::new(),
        StableUserRepository::new()
    )
}

#[query]
fn fetch_paper(paper_id: String) -> Result<Paper, String> {
    let controller = controller();

    controller.fetch(&paper_id)
}

#[query]
fn fetch_paper_as_author(paper_id: String) -> Result<Paper, String> {
    let controller = controller();

    controller.fetch_as_author(&paper_id, caller().into())
}

#[query]
fn fetch_all_paper_summaries() -> Vec<PaperSummaryDto> {
    let controller = controller();

    controller.fetch_all_summaries()
}

#[update(guard = "caller_is_user")]
fn update_paper(paper: Paper) -> Result<(), String> {
    let mut controller = controller();

    controller.update(caller().into(), paper)
}

#[update(guard = "caller_is_user")]
fn create_draft() -> String {
    let mut controller = controller();

    controller.create_draft(caller().into()).unwrap()
}
