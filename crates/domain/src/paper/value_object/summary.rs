use crate::user::UserPrimaryKey;

use super::PaperId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PaperSummary<U>
where
    U: UserPrimaryKey
{
    pub id: PaperId,
    pub lead_author: U,
}
