use crate::{
    controller::article::ArticleController,
    infrastructure::{
        article::repository::StableArticleRepository,
        user::repository::StableUserRepository,
    },
};
use domain::{article::{
    entity::dto::Article,
    value_object::ArticleId,
}, UserPrincipal};
use ic_cdk_macros::*;
use interface::article::*;

fn controller() -> ArticleController<StableArticleRepository, StableUserRepository, UserPrincipal> {
    ArticleController::<StableArticleRepository, StableUserRepository, UserPrincipal>::new(
        StableArticleRepository::new(),
        StableUserRepository::new()
    )
}

#[query]
fn fetch_article(article_id: ArticleId) -> Result<Article, String> {
    let controller = controller();

    controller.fetch(&article_id)
}
