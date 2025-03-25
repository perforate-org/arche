use domain::{paper::{
    entity::{dto, model::Paper}, repository::PaperRepository, service::PaperService
}, user::{repository::UserRepository, service::UserService, UserPrimaryKey}, PaperId};
use super::PaperController;
use std::str::FromStr;
use interface::paper::*;

impl<A, U, UK> PaperController<A, U, UK>
where
    A: PaperRepository<UserPrimaryKey = UK> + Clone,
    U: UserRepository<PrimaryKey = UK> + Clone,
    UK: UserPrimaryKey,
{
    pub fn create_draft(&mut self, user: UK) -> Result<String, String> {
        let mut service = PaperService::new(self.repository.clone());
        let draft_id = service.create_draft(user);
        let mut user_service = UserService::new(self.user_repository.clone());
        user_service.add_paper_as_lead_author(&user, &draft_id).map_err(|e| format!("Failed to add paper as lead author: {}", e))?;
        Ok(draft_id.to_string())
    }

    pub fn update(&mut self, user: UK, paper: dto::Paper) -> Result<(), String> {
        let id = PaperId::from_str(&paper.id).map_err(|e| format!("Invalid paper ID: {}", e))?;

        let mut service = PaperService::new(self.repository.clone());
        if !service.check_is_author(&user, &id) {
            return Err("User is not an author of the paper".to_string());
        }
        let model = paper.into_model(&self.user_repository).ok_or("Failed to convert paper to model")?;
        service.update(model).map_err(|e| format!("Failed to update paper: {}", e))
    }
}
