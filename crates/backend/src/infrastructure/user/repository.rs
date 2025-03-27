use crate::infrastructure::STATE;
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
        STATE.with_borrow(|s| s.users.get(&primary_key.as_principal())).map(|u| User::from_dao_with_id(u, Some(*user_id)))
    }

    fn get_by_primary_key(&self, primary_key: &UserPrincipal) -> Option<User> {
        STATE.with_borrow(|s| s.users.get(&primary_key.as_principal())).map(|u| User::from_dao(u, primary_key, self))
    }

    fn get_name(&self, user_id: &UserId) -> Option<domain::UserName> {
        let key = self.get_primary_key(user_id)?;
        self.get_name_by_primary_key(&key)
    }

    fn get_name_by_primary_key(&self, primary_key: &Self::PrimaryKey) -> Option<domain::UserName> {
        STATE.with_borrow(|s| s.user_names.get(primary_key).cloned())
    }

    fn contains(&self, primary_key: &UserPrincipal) -> bool {
        STATE.with_borrow(|s| s.user_existence.contains(&primary_key.as_principal()))
    }

    fn contains_id(&self, user_id: &UserId) -> bool {
        STATE.with_borrow(|s| s.user_principals.contains_key(user_id))
    }

    fn get_primary_key(&self, user_id: &UserId) -> Option<UserPrincipal> {
        STATE.with_borrow(|s| s.user_principals.get(user_id).copied())
    }

    fn get_user_id(&self, primary_key: &UserPrincipal) -> Option<UserId> {
        STATE.with_borrow(|s| s.user_ids.get(&primary_key.as_principal()).copied())
    }

    fn add(&mut self, primary_key: UserPrincipal, user: User) -> Result<(), UserRepositoryError> {
        let principal = primary_key.as_principal();

        if self.contains(&principal) {
            return Err(UserRepositoryError::PrimaryKeyAlreadyExists);
        }

        STATE.with_borrow_mut(|s| {
            s.user_names.insert(principal, user.name.clone());
            s.users.insert(principal, user.into());
            s.user_existence.insert(principal);
        });

        Ok(())
    }

    fn update(&mut self, primary_key: &UserPrincipal, user: User) -> Result<(), UserRepositoryError> {
        if !self.contains(primary_key) {
            return Err(UserRepositoryError::NotFound);
        }

        self.update_id(primary_key, user.id)?;

        STATE.with_borrow_mut(|s| {
            s.user_names.insert(primary_key.as_principal(), user.name.clone());
            s.users.insert(primary_key.as_principal(), user.into());
        });

        Ok(())
    }

    fn update_id(&mut self, primary_key: &UserPrincipal, new_id: Option<UserId>) -> Result<(), UserRepositoryError> {
        let principal = primary_key.as_principal();
        let old_id = self.get_user_id(primary_key);

        // Early return if IDs are the same
        if new_id == old_id {
            return Ok(());
        }

        match new_id {
            Some(new_id) => {
                // Check if new ID already exists
                if STATE.with_borrow(|s| s.user_principals.contains_key(&new_id)) {
                    return Err(UserRepositoryError::IdAlreadyExists);
                }

                // Update both primary and backup mappings atomically
                let update_mappings = || {
                    STATE.with_borrow_mut(|s| {
                        // Add new mappings
                        s.user_principals.insert(new_id, principal);
                        s.user_ids.insert(principal, new_id);

                        // Remove old ID if it exists
                        if let Some(old_id) = old_id {
                            s.user_principals.remove(&old_id);
                        }
                    });
                };

                update_mappings();
            }
            None => {
                if let Some(old_id) = old_id {
                    // Remove all mappings atomically when clearing ID
                    let clear_mappings = || {
                        STATE.with_borrow_mut(|s| {
                            s.user_principals.remove(&old_id);
                            s.user_ids.remove(&principal);
                        });
                    };

                    clear_mappings();
                }
            }
        }

        Ok(())
    }

    fn remove(&mut self, primary_key: &UserPrincipal) -> Result<User, UserRepositoryError> {
        let principal = primary_key.as_principal();

        STATE.with_borrow_mut(|s| {
            s.user_existence.remove(&principal);
            let id = s.user_ids.remove(&principal);

            if let Some(id) = id {
                s.user_principals.remove(&id).ok_or(UserRepositoryError::NotFound)?;
            }

            s.user_names.remove(&principal);
            s.users.remove(&principal).ok_or(UserRepositoryError::NotFound).map(|u| {
                User::from_dao_with_id(u, id)
            })
        })
    }
}
