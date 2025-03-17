use crate::infrastructure::{USERS, USER_PRINCIPALS, USER_PRINCIPALS_BACKUP, USER_IDS, USER_IDS_BACKUP};
use domain::user::{
    entity::model::User, value_object::{UserId, UserPrincipal, UserPrimaryKey},
};
pub use domain::user::repository::{UserRepository, UserRepositoryError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StableUserRepository;

impl StableUserRepository {
    pub fn new() -> Self {
        StableUserRepository
    }
}

impl UserRepository for StableUserRepository {
    type PrimaryKey = UserPrincipal;

    fn new() -> Self where Self: Sized {
        StableUserRepository::new()
    }

    fn get(&self, user_id: &UserId) -> Option<User> {
        let primary_key = self.get_primary_key(user_id)?;
        self.get_by_primary_key(&primary_key)
    }

    fn get_by_primary_key(&self, primary_key: &UserPrincipal) -> Option<User> {
        USERS.with_borrow(|map| map.get(&primary_key.as_principal())).map(|u| u.into())
    }

    fn contains(&self, user_id: &UserId) -> bool {
        USER_PRINCIPALS.with_borrow(|map| map.contains_key(user_id))
    }

    fn contains_by_primary_key(&self, primary_key: &UserPrincipal) -> bool {
        USER_IDS.with_borrow(|map| map.contains_key(&primary_key.as_principal()))
    }

    fn get_primary_key(&self, user_id: &UserId) -> Option<UserPrincipal> {
        USER_PRINCIPALS.with_borrow(|map| map.get(user_id).copied())
    }

    fn get_user_id(&self, primary_key: &UserPrincipal) -> Option<UserId> {
        USER_IDS.with_borrow(|map| map.get(&primary_key.as_principal()).copied())
    }

    fn add(&mut self, primary_key: UserPrincipal, id: UserId, user: User) -> Result<(), UserRepositoryError> {
        let principal = primary_key.as_principal();

        if self.contains(&id) {
            return Err(UserRepositoryError::IdAlreadyExists);
        }

        if self.contains_by_primary_key(&principal) {
            return Err(UserRepositoryError::PrimaryKeyAlreadyExists);
        }

        USER_PRINCIPALS.with_borrow_mut(|map| map.insert(id, principal));
        USER_PRINCIPALS_BACKUP.with_borrow_mut(|map| map.insert(id, principal));
        USER_IDS.with_borrow_mut(|map| map.insert(principal, id));
        USER_IDS_BACKUP.with_borrow_mut(|map| map.insert(principal, id));
        USERS.with_borrow_mut(|map| map.insert(principal, user.into()));

        Ok(())
    }

    fn update(&mut self, primary_key: &UserPrincipal, user: User) -> Result<(), UserRepositoryError> {
        if !self.contains_by_primary_key(primary_key) {
            return Err(UserRepositoryError::NotFound);
        }

        USERS.with_borrow_mut(|map| map.insert(primary_key.as_principal(), user.into()));

        Ok(())
    }

    fn change_id(&mut self, primary_key: &UserPrincipal, new_id: UserId) -> Result<(), UserRepositoryError> {
        let principal = primary_key.as_principal();

        let old_id = USER_IDS.with_borrow(|map| map.get(&principal).copied()).ok_or(UserRepositoryError::NotFound)?;
        if self.contains(&new_id) {
            return Err(UserRepositoryError::IdAlreadyExists);
        }

        USER_PRINCIPALS.with_borrow_mut(|map| map.insert(new_id, principal));
        USER_PRINCIPALS_BACKUP.with_borrow_mut(|map| map.insert(new_id, principal));
        USER_IDS.with_borrow_mut(|map| map.insert(principal, new_id));
        USER_IDS_BACKUP.with_borrow_mut(|map| map.insert(principal, new_id));

        USER_PRINCIPALS.with_borrow_mut(|map| map.remove(&old_id));
        USER_PRINCIPALS_BACKUP.with_borrow_mut(|map| map.remove(&old_id));

        Ok(())
    }

    fn remove(&mut self, primary_key: &UserPrincipal) -> Result<User, UserRepositoryError> {
        let principal = primary_key.as_principal();

        let id = USER_IDS.with_borrow_mut(|map| map.remove(&principal)).ok_or(UserRepositoryError::NotFound)?;
        USER_IDS_BACKUP.with_borrow_mut(|map| map.remove(&principal)).ok_or(UserRepositoryError::NotFound)?;

        USER_PRINCIPALS.with_borrow_mut(|map| map.remove(&id)).ok_or(UserRepositoryError::NotFound)?;
        USER_PRINCIPALS_BACKUP.with_borrow_mut(|map| map.remove(&id)).ok_or(UserRepositoryError::NotFound)?;

        USERS.with_borrow_mut(|map| map.remove(&principal).ok_or(UserRepositoryError::NotFound)).map(|u| u.into())
    }
}
