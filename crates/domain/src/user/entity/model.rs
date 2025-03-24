use crate::{
    paper::PaperId,
    user::{repository::UserRepository, UserId, UserName},
};
use super::dao::{UserDao, UserDaoVersion, V1};

#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: Option<UserId>,
    pub name: UserName,
    pub lead_authored_papers: Vec<PaperId>,
    pub co_authored_papers: Vec<PaperId>,
}

impl From<User> for UserDao {
    fn from(user: User) -> Self {
        UserDao {
            version: UserDaoVersion::V1(V1 {
                name: user.name,
                lead_authored_papers: user.lead_authored_papers,
                co_authored_papers: user.co_authored_papers,
            }),
        }
    }
}

impl User {
    pub fn from_dao<R: UserRepository>(dao: UserDao, primary_key: &R::PrimaryKey, repository: &R) -> Self {
        let id = repository.get_user_id(primary_key);

        match dao.version {
            UserDaoVersion::V1(v1) => User {
                id,
                name: v1.name,
                lead_authored_papers: v1.lead_authored_papers,
                co_authored_papers: v1.co_authored_papers,
            },
        }
    }

    pub fn from_dao_with_id(dao: UserDao, id: Option<UserId>) -> Self {
        match dao.version {
            UserDaoVersion::V1(v1) => User {
                id,
                name: v1.name,
                lead_authored_papers: v1.lead_authored_papers,
                co_authored_papers: v1.co_authored_papers,
            },
        }
    }

    pub fn add_lead_authored_paper(&mut self, paper_id: PaperId) {
        self.lead_authored_papers.push(paper_id);
    }

    pub fn add_co_authored_paper(&mut self, paper_id: PaperId) {
        self.co_authored_papers.push(paper_id);
    }
}
