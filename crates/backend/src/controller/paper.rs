use crate::use_case::paper::PaperUseCase;
use domain::{
    paper::{
        entity::dto,
        repository::PaperRepository,
    },
    user::{repository::UserRepository, UserPrimaryKey}
};
use interface::paper::*;

pub struct PaperController<A: PaperRepository, U: UserRepository, UK: UserPrimaryKey> {
    use_case: PaperUseCase<A, U, UK>,
}

impl<R: PaperRepository, U: UserRepository, UK: UserPrimaryKey> PaperController<R, U, UK>
where
    R: PaperRepository<UserPrimaryKey = UK> + Clone,
    U: UserRepository<PrimaryKey = UK> + Clone,
{
    pub fn new(repository: R, user_repository: U) -> Self {
        Self {
            use_case: PaperUseCase::new(repository, user_repository)
        }
    }

    pub fn fetch(&self, paper_id: &str) -> Result<dto::Paper, String> {
        let (paper, paper_id) = self.use_case.get_paper(paper_id)?;

        dto::Paper::from_model(paper, self.use_case.get_user_repository(), paper_id)
            .ok_or_else(|| format!("Failed to convert Paper model to DTO for paper: {}", paper_id))
    }

    pub fn fetch_as_author(&self, paper_id: &str, caller: UK) -> Result<dto::Paper, String> {
        // Check author permissions to retrieve models
        let (paper, paper_id) = self.use_case.get_paper_as_author(paper_id, caller)?;

        dto::Paper::from_model(paper, self.use_case.get_user_repository(), paper_id)
            .ok_or_else(|| format!("Failed to convert Paper model to DTO for paper: {}", paper_id))
    }

    pub fn fetch_all_summaries(&self) -> Vec<PaperSummaryDto> {
        let summaries = self.use_case.get_all_paper_summaries();

        summaries.into_iter().map(|(id, title, author, author_name)| {
            let author_id = self.use_case.get_user_repository().get_user_id(&author);
            let lead_author_id = match author_id {
                Some(id) => id.to_string(),
                None => format!("p_{}", author),
            };

            PaperSummaryDto {
                id,
                title,
                lead_author_id,
                lead_author_name: author_name,
            }
        }).collect()
    }

    pub fn create_draft(&mut self, user: UK) -> Result<String, String> {
        let draft_id = self.use_case.create_draft(user)?;
        Ok(draft_id.to_string())
    }

    pub fn update(&mut self, user: UK, paper_dto: dto::Paper) -> Result<(), String> {
        let model = paper_dto.into_model(self.use_case.get_user_repository())
            .ok_or("Failed to convert paper to model")?;

        self.use_case.update_paper(user, &paper_dto.id, model)
    }

    pub fn delete(&mut self, user: UK, paper_id: &str) -> Result<(), String> {
        self.use_case.delete_paper(user, paper_id)
    }
}
