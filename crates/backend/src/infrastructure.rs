use domain::{
    user::{
        entity::dao::UserDao,
        value_object::{UserId, UserPrincipal}
    },
    article::{
        entity::dao::ArticleDao,
        value_object::ArticleId,
    }
};
use crate::{
    infrastructure::article::repository::ArticleCounter,
    log::Log,
};
use ic_cdk::api::print;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, StableLog,
};
use std::{cell::RefCell, collections::HashMap, sync::Mutex};

pub mod article;
pub mod user;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static LOG: RefCell<StableLog<Log, Memory, Memory>> = RefCell::new({
        let result = StableLog::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        );

        match result {
            Ok(log) => log,
            Err(e) => {
                print(format!("Failed to initialize log: {}", e));
                ic_cdk::trap("Critical error: Failed to initialize stable log");
            }
        }
    });

    pub static USER_PRINCIPALS: RefCell<HashMap<UserId, UserPrincipal>> = RefCell::new(HashMap::new());

    pub static USER_PRINCIPALS_BACKUP: RefCell<StableBTreeMap<UserId, UserPrincipal, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
        )
    );

    pub static USER_IDS: RefCell<HashMap<UserPrincipal, UserId>> = RefCell::new(HashMap::new());

    pub static USER_IDS_BACKUP: RefCell<StableBTreeMap<UserPrincipal, UserId, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
        )
    );

    pub static USERS: RefCell<StableBTreeMap<UserPrincipal, UserDao, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))),
        )
    );

    pub static ARTICLE_COUNTER: RefCell<Mutex<ArticleCounter>> = RefCell::new(Mutex::new(ArticleCounter::default()));

    pub static ARTICLES: RefCell<StableBTreeMap<ArticleId, ArticleDao<UserPrincipal>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))),
        )
    );
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    use crate::log;

    // Save USER_PRINCIPALS, USER_IDS, and ARTICLE_COUNTER to stable storage.
    let save_result = ARTICLE_COUNTER.with_borrow(|article_counter| {
        USER_PRINCIPALS.with_borrow(|user_principals| {
            USER_IDS.with_borrow(|user_ids| {
                // Save all three structures in a tuple
                ic_cdk::storage::stable_save((
                    *article_counter.lock().unwrap(),
                    user_principals.clone(),
                    user_ids.clone()
                ))
            })
        })
    });

    // Handle error gracefully, avoiding unwrap()
    match save_result {
        Ok(_) => {
            ic_cdk::print("Successfully saved ARTICLE_COUNTER, USER_PRINCIPALS, and USER_IDS to stable storage");
        },
        Err(e) => {
            ic_cdk::print(format!("Failed to save data to stable storage: {:?}", e));
            let _ = log::error(
                "Failed to save data to stable storage",
                &format!("{:?}", e),
                "pre_upgrade",
            );
        }
    }
}


#[ic_cdk::post_upgrade]
fn post_upgrade() {
    use domain::{UserId, UserPrincipal};
    use std::sync::Mutex;

    // Load both USER_PRINCIPALS and USER_IDS maps from stable storage.
    let restore_result = ic_cdk::storage::stable_restore::<
        (ArticleCounter, HashMap<UserId, UserPrincipal>, HashMap<UserPrincipal, UserId>)
    >();

    // Handle error gracefully, avoiding unwrap()
    match restore_result {
        Ok((old_article_counter, old_user_principals, old_user_ids)) => {
            // Restore ARTICLE_COUNTER
            ARTICLE_COUNTER.with_borrow_mut(|counter| {
                *counter = Mutex::new(old_article_counter);
            });

            // Restore USER_PRINCIPALS
            USER_PRINCIPALS.with_borrow_mut(|map| {
                *map = old_user_principals;
            });

            // Restore USER_IDS
            USER_IDS.with_borrow_mut(|map| {
                *map = old_user_ids;
            });

            ic_cdk::print("Successfully restored ARTICLE_COUNTER, USER_PRINCIPALS, and USER_IDS from stable storage");
        },
        Err(e) => {
            ic_cdk::print(format!("Failed to restore data from stable storage: {:?}", e));

            // Initialize with empty maps to ensure system can still function
            USER_PRINCIPALS.with_borrow_mut(|map| {
                map.clear();
            });

            USER_IDS.with_borrow_mut(|map| {
                map.clear();
            });

            // Restore ARTICLE_COUNTER
            ARTICLE_COUNTER.with_borrow_mut(|counter| {
                *counter = {
                    Mutex::new(
                        ARTICLES.with_borrow(|articles| {
                            match articles.keys().last() {
                                Some(key) => {
                                    ArticleCounter {
                                        last_generated_months: key.months(),
                                        count_in_month: key.number(),
                                    }
                                },
                                None => ArticleCounter::default()
                            }
                        })
                    )
                };
            });

            // Repopulate USER_PRINCIPALS from backup if available
            USER_PRINCIPALS_BACKUP.with_borrow(|backup| {
                USER_PRINCIPALS.with_borrow_mut(|map| {
                    for (user_id, principal) in backup.iter() {
                        map.insert(user_id, principal);
                    }
                });
            });

            // Repopulate USER_IDS from backup if available
            USER_IDS_BACKUP.with_borrow(|backup| {
                USER_IDS.with_borrow_mut(|map| {
                    for (principal, user_id) in backup.iter() {
                        map.insert(principal, user_id);
                    }
                });
            });

            ic_cdk::print("Restored ARTICLE_COUNTER, USER_PRINCIPALS, and USER_IDS from backups");
        }
    }
}
