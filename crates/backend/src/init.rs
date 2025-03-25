use std::str::FromStr;

use candid::Principal;
use domain::{
    paper::{
        entity::model::Paper,
        repository::PaperRepository, PaperCategory,
        value_object::{PaperId, PaperTitle, PaperContent, ContentFormat, ContentSource},
    },
    user::{
        entity::model::User,
        repository::UserRepository,
        value_object::{UserId, UserName, UserPrincipal},
    },
};
use crate::infrastructure::{
    paper::repository::StablePaperRepository,
    user::repository::StableUserRepository,
};

pub(super) fn init() {
    let mut user_repo = StableUserRepository::new();
    let mut paper_repo = StablePaperRepository::new();

    let anonymous_principal: UserPrincipal = Principal::anonymous().into();
    let anonymous_id = UserId::new("anonymous").unwrap();
    let anonymous_name = UserName::new("John Doe").unwrap();

    let sample_id = PaperId::from_str("2025-01-0001").unwrap();

    let anonymous_user = User {
        name: anonymous_name,
        lead_authored_papers: vec![sample_id],
        ..Default::default()
    };

    let sample_title = PaperTitle::new("Lorem Ipsum").unwrap();
    let sample_category = PaperCategory::Blockchain;
        let sample_content = PaperContent::new(
            ContentFormat::Text,
            ContentSource::Raw(
                r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent luctus ultrices felis, ut consequat dolor laoreet quis. Duis in lorem at velit condimentum elementum ut nec dolor. Nullam accumsan vehicula dolor, et maximus tortor ultricies quis. Quisque varius non augue ac sodales. Proin odio diam, gravida non libero nec, egestas facilisis enim. Nunc malesuada leo vel vestibulum euismod. Ut commodo vulputate lectus. Vivamus congue laoreet arcu vitae varius. Morbi et nulla quam. Cras semper ex at mauris auctor, vel consequat sapien rutrum. Interdum et malesuada fames ac ante ipsum primis in faucibus. Praesent varius varius mi id commodo. Lorem ipsum dolor sit amet, consectetur adipiscing elit.

    Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Proin quam dolor, scelerisque quis arcu eu, laoreet interdum purus. In quis pellentesque ex. Donec pharetra aliquam interdum. Vestibulum vitae accumsan magna. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Nunc sollicitudin eget neque a egestas. Integer rutrum tristique leo at ornare. Morbi pretium nulla ut egestas porta.

    Nam mattis turpis non lacus convallis mattis. Proin lacus tortor, pretium et tellus lacinia, convallis porttitor lorem. Fusce et massa fringilla, accumsan massa sollicitudin, sollicitudin ipsum. Pellentesque turpis sem, facilisis quis ante quis, venenatis faucibus nibh. Quisque id pretium enim. Morbi iaculis neque ac tincidunt interdum. Fusce rutrum dolor augue, nec molestie nisl pulvinar tristique.

    Integer lacus augue, varius varius commodo in, facilisis eget lacus. Nullam quis tincidunt tortor. Aenean vel lacinia justo. Aliquam a venenatis felis. Ut euismod ligula in mauris commodo vestibulum. Maecenas id feugiat magna. Nullam id laoreet dui, quis scelerisque nisl. Phasellus gravida mi vitae efficitur finibus. Vivamus tincidunt sem elit, et malesuada nisi imperdiet sit amet. Fusce imperdiet lorem a dolor vestibulum, et ullamcorper urna gravida. Aliquam eget egestas sem. Proin et elit non ex finibus pulvinar at nec lectus. Aliquam elit urna, pulvinar at metus sed, lobortis lobortis neque. Praesent ultrices ante a tortor euismod rutrum. Praesent vel lobortis magna.

    Proin tempor metus eu vehicula hendrerit. Praesent tempus, velit quis egestas pharetra, dolor nulla varius lacus, in sodales diam sapien at sapien. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Duis bibendum velit risus, vel bibendum purus congue et. Suspendisse a felis porta, sagittis ligula ac, rhoncus lectus. Ut eget lectus quis leo malesuada iaculis. Phasellus porttitor sapien in ante ullamcorper malesuada. Sed ligula mi, posuere id lacinia ut, rhoncus vel ex. Donec vel nunc felis. Integer eu enim dolor. Nullam accumsan mauris nec eleifend eleifend. Proin condimentum arcu ex, quis porta elit tincidunt vel."#
                .into()
            )
        );
    let mut sample_paper = Paper::new_draft(anonymous_principal, &mut paper_repo);
    sample_paper.title = sample_title;
    sample_paper.ab = r#"Lorem ipsum is typically a corrupted version of De finibus bonorum et malorum, a 1st-century BC text by the Roman statesman and philosopher Cicero, with words altered, added, and removed to make it nonsensical and improper Latin. The first two words themselves are a truncation of dolorem ipsum ("pain itself")."#.to_string();
    sample_paper.content = sample_content;
    sample_paper.categories.push(sample_category);
    sample_paper.tags.push("Tag1".to_string());
    sample_paper.tags.push("Tag2".to_string());
    let _ = sample_paper.publish();

    let _ = user_repo.add(anonymous_principal, anonymous_user);
    let _ = user_repo.update_id(&anonymous_principal, Some(anonymous_id));
    let _ = paper_repo.insert(sample_id, sample_paper);
}
