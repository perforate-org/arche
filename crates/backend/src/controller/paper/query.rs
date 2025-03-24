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
        let res = dto::Paper::from_model(paper, &self.user_repository)
            .ok_or_else(|| format!("Failed to convert Paper model to DTO for paper: {}", paper_id))?;
        Ok(res)
    }
}
