use crate::infrastructure::{USERS, USER_EXISTENCE, USER_IDS, USER_IDS_BACKUP, USER_PRINCIPALS, USER_PRINCIPALS_BACKUP};
use domain::user::{
    entity::model::User, value_object::{UserId, UserPrimaryKey, UserPrincipal},
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
        USERS.with_borrow(|map| map.get(&primary_key.as_principal())).map(|u| User::from_dao_with_id(u, Some(*user_id)))
    }

    fn get_by_primary_key(&self, primary_key: &UserPrincipal) -> Option<User> {
        USERS.with_borrow(|map| map.get(&primary_key.as_principal())).map(|u| User::from_dao(u, primary_key, self))
    }

    fn contains(&self, primary_key: &UserPrincipal) -> bool {
        USER_EXISTENCE.with_borrow(|map| map.contains(&primary_key.as_principal()))
    }

    fn contains_id(&self, user_id: &UserId) -> bool {
        USER_PRINCIPALS.with_borrow(|map| map.contains_key(user_id))
    }

    fn get_primary_key(&self, user_id: &UserId) -> Option<UserPrincipal> {
        USER_PRINCIPALS.with_borrow(|map| map.get(user_id).copied())
    }

    fn get_user_id(&self, primary_key: &UserPrincipal) -> Option<UserId> {
        USER_IDS.with_borrow(|map| map.get(&primary_key.as_principal()).copied())
    }

    fn add(&mut self, primary_key: UserPrincipal, user: User) -> Result<(), UserRepositoryError> {
        let principal = primary_key.as_principal();

        if self.contains(&principal) {
            return Err(UserRepositoryError::PrimaryKeyAlreadyExists);
        }

        USERS.with_borrow_mut(|map| map.insert(principal, user.into()));
        USER_EXISTENCE.with_borrow_mut(|set| set.insert(principal));

        Ok(())
    }

    fn update(&mut self, primary_key: &UserPrincipal, user: User) -> Result<(), UserRepositoryError> {
        if !self.contains(primary_key) {
            return Err(UserRepositoryError::NotFound);
        }

        self.update_id(primary_key, user.id)?;
        USERS.with_borrow_mut(|map| map.insert(primary_key.as_principal(), user.into()));

        Ok(())
    }

    fn update_id(&mut self, primary_key: &UserPrincipal, new_id: Option<UserId>) -> Result<(), UserRepositoryError> {
        let principal = primary_key.as_principal();
        let old_id = USER_IDS.with_borrow(|map| map.get(&principal).copied());

        // Early return if IDs are the same
        if new_id == old_id {
            return Ok(());
        }

        match new_id {
            Some(new_id) => {
                // Check if new ID already exists
                if USER_PRINCIPALS.with_borrow(|map| map.contains_key(&new_id)) {
                    return Err(UserRepositoryError::IdAlreadyExists);
                }

                // Update both primary and backup mappings atomically
                let update_mappings = || {
                    // Add new mappings
                    USER_PRINCIPALS.with_borrow_mut(|map| map.insert(new_id, principal));
                    USER_PRINCIPALS_BACKUP.with_borrow_mut(|map| map.insert(new_id, principal));
                    USER_IDS.with_borrow_mut(|map| map.insert(principal, new_id));
                    USER_IDS_BACKUP.with_borrow_mut(|map| map.insert(principal, new_id));

                    // Remove old ID if it exists
                    if let Some(old_id) = old_id {
                        USER_PRINCIPALS.with_borrow_mut(|map| map.remove(&old_id));
                        USER_PRINCIPALS_BACKUP.with_borrow_mut(|map| map.remove(&old_id));
                    }
                };

                update_mappings();
            }
            None => {
                if let Some(old_id) = old_id {
                    // Remove all mappings atomically when clearing ID
                    let clear_mappings = || {
                        USER_PRINCIPALS.with_borrow_mut(|map| map.remove(&old_id));
                        USER_PRINCIPALS_BACKUP.with_borrow_mut(|map| map.remove(&old_id));
                        USER_IDS.with_borrow_mut(|map| map.remove(&principal));
                        USER_IDS_BACKUP.with_borrow_mut(|map| map.remove(&principal));
                    };

                    clear_mappings();
                }
            }
        }

        Ok(())
    }

    fn remove(&mut self, primary_key: &UserPrincipal) -> Result<User, UserRepositoryError> {
        let principal = primary_key.as_principal();

        USER_EXISTENCE.with_borrow_mut(|map| map.remove(&principal));

        let id = USER_IDS.with_borrow_mut(|map| map.remove(&principal));
        USER_IDS_BACKUP.with_borrow_mut(|map| map.remove(&principal));

        if let Some(id) = id {
            USER_PRINCIPALS.with_borrow_mut(|map| map.remove(&id)).ok_or(UserRepositoryError::NotFound)?;
            USER_PRINCIPALS_BACKUP.with_borrow_mut(|map| map.remove(&id)).ok_or(UserRepositoryError::NotFound)?;
        }
        USERS.with_borrow_mut(|map| map.remove(&principal).ok_or(UserRepositoryError::NotFound)).map(|u| {
            User::from_dao_with_id(u, id)
        })
    }
}
