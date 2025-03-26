use domain::{
    paper::{
        entity::model::Paper,
        repository::PaperRepository,
        service::PaperService,
        PaperId,
    },
    user::{repository::UserRepository, service::UserService, UserPrimaryKey},
};
use std::marker::PhantomData;
use std::str::FromStr;

pub struct PaperUseCase<A: PaperRepository, U: UserRepository, UK: UserPrimaryKey> {
    repository: A,
    user_repository: U,
    _marker: PhantomData<UK>,
}

impl<R: PaperRepository, U: UserRepository, UK: UserPrimaryKey> PaperUseCase<R, U, UK>
where
    R: PaperRepository<UserPrimaryKey = UK> + Clone,
    U: UserRepository<PrimaryKey = UK> + Clone,
    UK: UserPrimaryKey,
{
    pub fn new(repository: R, user_repository: U) -> Self {
        Self {
            repository,
            user_repository,
            _marker: PhantomData,
        }
    }

    // Return domain model
    pub fn get_paper(&self, paper_id_str: &str) -> Result<(Paper<UK>, PaperId), String> {
        let paper_id = PaperId::from_str(paper_id_str)
            .map_err(|e| format!("Invalid paper ID: {}", e))?;

        let paper = self.repository.get(&paper_id)
            .ok_or_else(|| format!("Paper not found: {}", paper_id))?;

        Ok((paper, paper_id))
    }

    // Checks for authorship and then returns a domain model.
    pub fn get_paper_as_author(&self, paper_id_str: &str, caller: UK) -> Result<(Paper<UK>, PaperId), String> {
        let paper_id = PaperId::from_str(paper_id_str)
            .map_err(|e| format!("Invalid paper ID: {}", e))?;

        // Author Authority Check
        let summary = self.repository.get_summary(&paper_id);
        if let Some(summary) = summary {
            if summary.lead_author != caller {
                return Err(format!("User is not the lead author of paper: {}", paper_id));
            }
        }

        let paper = self.repository.get(&paper_id)
            .ok_or_else(|| format!("Paper not found: {}", paper_id))?;

        Ok((paper, paper_id))
    }

    // Obtaining paper summary information
    pub fn get_all_paper_summaries(&self) -> Vec<(String, String, UK, String)> {
        let mut results = Vec::new();

        let summaries = self.repository.iter_summary();
        for summary in summaries {
            let (id, author) = (summary.id, summary.lead_author);
            let title = self.repository.get_title(&id);
            let author_name = self.user_repository.get_name_by_primary_key(&author);

            if let (Some(title), Some(author_name)) = (title, author_name) {
                results.push((
                    id.to_string(),
                    title.to_string(),
                    author,
                    author_name.to_string(),
                ));
            }
        }

        results
    }

    // Drafting
    pub fn create_draft(&mut self, user: UK) -> Result<PaperId, String> {
        let mut service = PaperService::new(self.repository.clone());
        let draft_id = service.create_draft(user);

        let mut user_service = UserService::new(self.user_repository.clone());
        user_service.add_paper_as_lead_author(&user, &draft_id)
            .map_err(|e| format!("Failed to add paper as lead author: {}", e))?;

        Ok(draft_id)
    }

    // Thesis Updates
    pub fn update_paper(&mut self, user: UK, id_str: &str, model: Paper<UK>) -> Result<(), String> {
        let id = PaperId::from_str(id_str)
            .map_err(|e| format!("Invalid paper ID: {}", e))?;

        let mut service = PaperService::new(self.repository.clone());
        if !service.check_is_author(&user, &id) {
            return Err("User is not an author of the paper".to_string());
        }

        service.update(model).map_err(|e| format!("Failed to update paper: {}", e))
    }

    // Get a reference to the user repository
    pub fn get_user_repository(&self) -> &U {
        &self.user_repository
    }

    pub fn delete_paper(&mut self, user: UK, paper_id_str: &str) -> Result<(), String> {
        let paper_id = PaperId::from_str(paper_id_str)
            .map_err(|e| format!("Invalid paper ID: {}", e))?;

        // Check if the paper exists
        if !self.repository.contains(&paper_id) {
            return Err(format!("Paper not found: {}", paper_id));
        }

        // Verify the caller is the lead author of the paper
        let summary = self.repository.get_summary(&paper_id)
            .ok_or_else(|| format!("Paper summary not found: {}", paper_id))?;

        if summary.lead_author != user {
            return Err("Only the lead author can delete the paper".to_string());
        }

        // Remove the paper from the author's list
        let mut user_service = UserService::new(self.user_repository.clone());
        user_service
            .remove_paper_as_lead_author(&user, &paper_id)
            .map_err(|e| format!("Failed to remove paper from user's list: {}", e))?;

        // Delete the paper from the repository
        self.repository.remove(&paper_id)
            .ok_or_else(|| format!("Failed to delete paper: {}", paper_id))?;

        Ok(())
    }
}
