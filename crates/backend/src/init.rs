use std::str::FromStr;

use candid::Principal;
use domain::{
    article::{
        entity::model::Article,
        repository::ArticleRepository, ArticleCategory,
    },
    user::{
        entity::model::User,
        repository::UserRepository,
        value_object::{UserId, UserName, UserPrincipal},
    }, ArticleId, ArticleTitle,
};
use crate::infrastructure::{
    article::repository::StableArticleRepository,
    user::repository::StableUserRepository,
};

pub(super) fn init() {
    let anonymous_principal: UserPrincipal = Principal::anonymous().into();
    let anonymous_id = UserId::new("anonymous").unwrap();
    let anonymous_name = UserName::new("Anonymous User").unwrap();
    let anonymous_user = User::new(anonymous_name);

    let sample_id = ArticleId::from_str("2025-01-0001").unwrap();
    let sample_title = ArticleTitle::new("Sample Article").unwrap();
    let sample_category = ArticleCategory::Blockchain;
    let mut sample_article = Article::new_draft(anonymous_principal, sample_title, "Summary".to_string(), "Content".to_string(), vec![sample_category], vec!["Tag1".to_string(), "Tag2".to_string()]);
    let _ = sample_article.publish();

    let mut user_repo = StableUserRepository::new();
    let mut article_repo = StableArticleRepository::new();

    let _ = user_repo.add(anonymous_principal, anonymous_id, anonymous_user);
    let _ = article_repo.insert(sample_id, sample_article);
}
