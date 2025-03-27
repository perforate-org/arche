use crate::infrastructure::STATE;
use candid::CandidType;
use chrono::{DateTime, Datelike};
use domain::{
    paper::{
        PaperId, PaperSummary,
        entity::model::Paper,
        repository::PaperRepository,
    },
    user::UserPrincipal,
};
use serde::{Deserialize, Serialize};

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
        STATE.with_borrow(|s| s.papers.get(paper_id)).map(|a| Paper::from_dao(a, *paper_id))
    }

    fn get_summary(&self, paper_id: &PaperId) -> Option<domain::paper::PaperSummary<Self::UserPrimaryKey>> {
        let author = STATE.with_borrow(|s| s.paper_lead_authors.get(paper_id).copied())?;
        Some(PaperSummary {
            id: *paper_id,
            lead_author: author,
        })
    }

    fn get_title(&self, paper_id: &PaperId) -> Option<domain::PaperTitle> {
        STATE.with_borrow(|s| s.paper_titles.get(paper_id).cloned())
    }

    fn contains(&self, paper_id: &PaperId) -> bool {
        STATE.with_borrow(|s| s.papers.contains_key(paper_id))
    }

    fn iter_summary(&self) -> impl Iterator<Item = PaperSummary<Self::UserPrimaryKey>> {
        STATE.with_borrow(|s| {
            s.paper_lead_authors.iter()
                .map(|(id, lead_author)| PaperSummary { id: *id, lead_author: *lead_author })
                .collect::<Vec<_>>()
                .into_iter()
        })
    }

    fn insert(&mut self, paper_id: PaperId, paper: Paper<UserPrincipal>) -> Option<Paper<UserPrincipal>> {
        STATE.with_borrow_mut(|s| s.paper_titles.insert(paper_id, paper.title.clone()));
        STATE.with_borrow_mut(|s| s.paper_lead_authors.insert(paper_id, paper.lead_author));
        STATE.with_borrow_mut(|s| s.papers.insert(paper_id, paper.into())).map(|a| Paper::from_dao(a, paper_id))
    }

    fn remove(&mut self, paper_id: &PaperId) -> Option<Paper<UserPrincipal>> {
        STATE.with_borrow_mut(|s| s.paper_titles.remove(paper_id));
        STATE.with_borrow_mut(|s| s.paper_lead_authors.remove(paper_id));
        STATE.with_borrow_mut(|s| s.papers.remove(paper_id)).map(|a| Paper::from_dao(a, *paper_id))
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

        STATE.with_borrow_mut(|s| {
            let counter = s.paper_counter.get_mut().unwrap();

            // Reset counter if month has changed
            if months == counter.last_generated_months {
                counter.count_in_month += 1;
            } else {
                counter.last_generated_months = months;
                counter.count_in_month = 1;
            }

            PaperId::new(months, counter.count_in_month, 1).unwrap()
        })
    }
}

#[derive(Debug, Clone, Copy, Default, CandidType, Serialize, Deserialize)]
pub struct PaperCounter {
    pub last_generated_months: u16,
    pub count_in_month: u32,
}
