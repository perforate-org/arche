use crate::infrastructure::{PAPER_COUNTER, PAPERS};
use candid::CandidType;
use chrono::{DateTime, Datelike};
use domain::{
    paper::{
        PaperId,
        entity::model::Paper,
        repository::PaperRepository,
    },
    user::UserPrincipal,
};
use serde::Deserialize;

#[derive(Clone, Copy)]
pub struct StablePaperRepository;

impl StablePaperRepository {
    /// Creates a new instance of `StablePaperRepository`.
    pub fn new() -> Self {
        StablePaperRepository {}
    }
}

impl PaperRepository for StablePaperRepository {
    type UserPrimaryKey = UserPrincipal;

    fn get(&self, paper_id: &PaperId) -> Option<Paper<UserPrincipal>> {
        PAPERS.with_borrow(|papers| papers.get(paper_id)).map(|a| a.into())
    }

    fn contains(&self, paper_id: &PaperId) -> bool {
        PAPERS.with_borrow(|papers| papers.contains_key(paper_id))
    }

    fn insert(&mut self, paper_id: PaperId, paper: Paper<UserPrincipal>) -> Option<Paper<UserPrincipal>> {
        PAPERS.with_borrow_mut(|papers| papers.insert(paper_id, paper.into())).map(|a| a.into())
    }

    fn remove(&mut self, paper_id: &PaperId) -> Option<Paper<UserPrincipal>> {
        PAPERS.with_borrow_mut(|papers| papers.remove(paper_id)).map(|a| a.into())
    }

    fn generate_id(&mut self) -> PaperId {
        // Get datetime information
        let now = ic_cdk::api::time() as i64;
        let datetime = DateTime::from_timestamp_nanos(now);

        // Get year and month (elapsed since 1970)
        let year = datetime.year() as u16 - 1970;
        let month = datetime.month0() as u16;

        // Months since Unix epoch
        let months = year * 12 + month;

        PAPER_COUNTER.with_borrow_mut(|counter| {
            let counter_guard = counter.get_mut().unwrap();

            // Reset counter if month has changed
            if months == counter_guard.last_generated_months {
                counter_guard.count_in_month += 1;
            } else {
                counter_guard.last_generated_months = months;
                counter_guard.count_in_month = 1;
            }

            PaperId::new(months, counter_guard.count_in_month, 1).unwrap()
        })
    }
}

#[derive(Debug, Clone, Copy, Default, CandidType, Deserialize)]
pub struct PaperCounter {
    pub last_generated_months: u16,
    pub count_in_month: u32,
}
