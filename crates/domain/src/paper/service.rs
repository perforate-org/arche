use crate::paper::{
    entity::model::Paper,
    repository::PaperRepository,
    PaperId,
};

use super::PaperSummary;

pub struct PaperService<R>
where
    R: PaperRepository,
{
    repository: R,
}

impl<R> PaperService<R>
where
    R: PaperRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn get_papers_by_id(&self, ids: Vec<PaperId>) -> Vec<Option<Paper<R::UserPrimaryKey>>> {
        let mut papers = Vec::new();
        for id in ids {
            papers.push(self.repository.get(&id));
        }
        papers
    }

    pub fn get_all_summaries(&self) -> Vec<PaperSummary<R::UserPrimaryKey>> {
        self.repository.iter_summary().collect()
    }

    pub fn check_is_author(&self, user: &R::UserPrimaryKey, paper_id: &PaperId) -> bool {
        if let Some(summary) = self.repository.get_summary(paper_id) {
            summary.lead_author == *user
        } else {
            false
        }
    }

    pub fn update(&mut self, paper: Paper<R::UserPrimaryKey>) -> Result<(), String> {
        let id = paper.id;
        if self.repository.contains(&id) {
            self.repository.insert(id, paper);
            Ok(())
        } else {
            Err("Paper not found".to_string())
        }
    }

    pub fn create_draft(
        &mut self,
        lead_author: R::UserPrimaryKey,
    ) -> PaperId {
        let paper = Paper::new_draft(lead_author, &mut self.repository);
        let id = paper.id;
        let _ = self.repository.insert(id, paper);
        id
    }

    pub fn remove(&mut self, id: &PaperId) -> Result<(), String> {
        if self.repository.contains(id) {
            self.repository.remove(id);
            Ok(())
        } else {
            Err("Paper not found".to_string())
        }
    }
}
