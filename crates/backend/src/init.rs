use std::str::FromStr;

use candid::Principal;
use domain::{
    paper::{
        entity::model::Paper,
        repository::PaperRepository, PaperCategory,
    },
    user::{
        entity::model::User,
        repository::UserRepository,
        value_object::{UserId, UserName, UserPrincipal},
    }, PaperId, PaperTitle,
};
use crate::infrastructure::{
    paper::repository::StablePaperRepository,
    user::repository::StableUserRepository,
};

pub(super) fn init() {
    let anonymous_principal: UserPrincipal = Principal::anonymous().into();
    let anonymous_id = UserId::new("anonymous").unwrap();
    let anonymous_name = UserName::new("John Doe").unwrap();
    let anonymous_user = User {
        name: anonymous_name,
        ..Default::default()
    };

    let sample_id = PaperId::from_str("2025-01-0001").unwrap();
    let sample_title = PaperTitle::new("Sample Paper").unwrap();
    let sample_category = PaperCategory::Blockchain;
    let mut sample_paper = Paper::new_draft(anonymous_principal, sample_title, "Abstract".to_string(), "Content".to_string(), vec![sample_category], vec!["Tag1".to_string(), "Tag2".to_string()]);
    let _ = sample_paper.publish();

    let mut user_repo = StableUserRepository::new();
    let mut paper_repo = StablePaperRepository::new();

    let _ = user_repo.add(anonymous_principal, anonymous_user);
    let _ = user_repo.update_id(&anonymous_principal, Some(anonymous_id));
    let _ = paper_repo.insert(sample_id, sample_paper);
}
