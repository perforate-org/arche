use crate::infrastructure::{ARTICLE_COUNTER, ARTICLES};
use candid::CandidType;
use chrono::{DateTime, Datelike};
use domain::{
    article::{
        ArticleId,
        entity::model::Article,
        repository::ArticleRepository,
    },
    user::UserPrincipal,
};
use serde::Deserialize;

#[derive(Clone, Copy)]
pub struct StableArticleRepository;

impl StableArticleRepository {
    /// Creates a new instance of `StableArticleRepository`.
    pub fn new() -> Self {
        StableArticleRepository {}
    }
}

impl ArticleRepository for StableArticleRepository {
    type UserPrimaryKey = UserPrincipal;

    fn get(&self, article_id: &ArticleId) -> Option<Article<UserPrincipal>> {
        ARTICLES.with_borrow(|articles| articles.get(article_id)).map(|a| a.into())
    }

    fn contains(&self, article_id: &ArticleId) -> bool {
        ARTICLES.with_borrow(|articles| articles.contains_key(article_id))
    }

    fn insert(&mut self, article_id: ArticleId, article: Article<UserPrincipal>) -> Option<Article<UserPrincipal>> {
        ARTICLES.with_borrow_mut(|articles| articles.insert(article_id, article.into())).map(|a| a.into())
    }

    fn remove(&mut self, article_id: &ArticleId) -> Option<Article<UserPrincipal>> {
        ARTICLES.with_borrow_mut(|articles| articles.remove(article_id)).map(|a| a.into())
    }

    fn generate_id(&mut self) -> ArticleId {
        // Get datetime information
        let now = ic_cdk::api::time() as i64;
        let datetime = DateTime::from_timestamp_nanos(now);

        // Get year and month (elapsed since 1970)
        let year = datetime.year() as u16 - 1970;
        let month = datetime.month0() as u16;

        // Months since Unix epoch
        let months = year * 12 + month;

        ARTICLE_COUNTER.with_borrow_mut(|counter| {
            let counter_guard = counter.get_mut().unwrap();

            // Reset counter if month has changed
            if months == counter_guard.last_generated_months {
                counter_guard.count_in_month += 1;
            } else {
                counter_guard.last_generated_months = months;
                counter_guard.count_in_month = 1;
            }

            ArticleId::new(months, counter_guard.count_in_month, 1).unwrap()
        })
    }
}

#[derive(Debug, Clone, Copy, Default, CandidType, Deserialize)]
pub struct ArticleCounter {
    pub last_generated_months: u16,
    pub count_in_month: u32,
}
