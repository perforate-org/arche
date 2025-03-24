#![allow(unused)]

use crate::{
    controller::paper::PaperController,
    infrastructure::{
        paper::repository::StablePaperRepository,
        user::repository::StableUserRepository,
    },
};
use domain::{paper::entity::dto::Paper, UserPrincipal};
use ic_cdk_macros::*;
use interface::paper::*;

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
