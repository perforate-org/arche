use domain::{paper::{
    repository::PaperRepository,
    entity::dto,
    PaperId,
}, user::{repository::UserRepository, UserPrimaryKey}};
use super::PaperController;
use std::str::FromStr;
use interface::paper::*;

impl<A, U, UK> PaperController<A, U, UK>
where
    A: PaperRepository<UserPrimaryKey = UK> + Clone,
    U: UserRepository<PrimaryKey = UK> + Clone,
    UK: UserPrimaryKey,
{
    pub fn fetch(&self, paper_id: &str) -> Result<dto::Paper, String> {
        let paper_id = PaperId::from_str(paper_id).map_err(|e| format!("Invalid paper ID: {}", e))?;

        let paper = match self.repository.get(&paper_id) {
            Some(paper) => paper,
            None => return Err(format!("Paper not found: {}", paper_id)),
        };
        let res = dto::Paper::from_model(paper, &self.user_repository, paper_id)
            .ok_or_else(|| format!("Failed to convert Paper model to DTO for paper: {}", paper_id))?;
        Ok(res)
    }

    pub fn fetch_as_author(&self, paper_id: &str, caller: UK) -> Result<dto::Paper, String> {
        let paper_id = PaperId::from_str(paper_id).map_err(|e| format!("Invalid paper ID: {}", e))?;

        let summary = self.repository.get_summary(&paper_id);
        if let Some(summary) = summary {
            if summary.lead_author != caller {
                return Err(format!("User is not the lead author of paper: {}", paper_id));
            }
        }

        let paper = match self.repository.get(&paper_id) {
            Some(paper) => paper,
            None => return Err(format!("Paper not found: {}", paper_id)),
        };
        let res = dto::Paper::from_model(paper, &self.user_repository, paper_id)
            .ok_or_else(|| format!("Failed to convert Paper model to DTO for paper: {}", paper_id))?;
        Ok(res)
    }

    pub fn fetch_all_summaries(&self) -> Vec<PaperSummaryDto> {
        let mut res = Vec::new();

        let summaries = self.repository.iter_summary();
        for summary in summaries {
            let (id, author) = (summary.id, summary.lead_author);
            let title = self.repository.get_title(&id);
            let author_name = self.user_repository.get_name_by_primary_key(&author);
            let author_id = self.user_repository.get_user_id(&author);
            if let (Some(title), Some(author_name)) = (title, author_name) {
                let lead_author_id = match author_id {
                    Some(id) => id.to_string(),
                    None => format!("p_{}", author),
                };
                res.push(PaperSummaryDto {
                    id: id.to_string(),
                    title: title.to_string(),
                    lead_author_id,
                    lead_author_name: author_name.to_string(),
                });
            }
        }

        res
    }
}
