use crate::paper::{
    entity::model::Paper,
    repository::PaperRepository,
    PaperId, PaperTitle, PaperCategory,
};

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

    pub fn create_draft(
        &mut self,
        lead_author: R::UserPrimaryKey,
        title: PaperTitle,
        ab: String,
        content: String,
        categories: Vec<PaperCategory>,
        tags: Vec<String>,
    ) -> PaperId {
        let id = self.repository.generate_id();
        let paper = Paper::new_draft(lead_author, title, ab, content, categories, tags);
        self.repository.insert(id, paper).unwrap();
        id
    }
}
