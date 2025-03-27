use domain::{
    paper::{
        entity::dao::PaperDao,
        value_object::PaperId,
    }, user::{
        entity::dao::UserDao,
        value_object::{UserId, UserPrincipal, UserName}
    }, PaperTitle
};
use crate::{
    infrastructure::paper::repository::PaperCounter,
    log::Log,
};
use ic_cdk::api::print;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, StableLog,
};
use std::{cell::RefCell, collections::{HashMap, HashSet, BTreeMap}, sync::Mutex};

pub mod paper;
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

    pub static USER_EXISTENCE: RefCell<HashSet<UserPrincipal>> = RefCell::new(HashSet::new());

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

    pub static USER_NAMES: RefCell<HashMap<UserPrincipal, UserName>> = RefCell::new(HashMap::new());

    pub static PAPER_COUNTER: RefCell<Mutex<PaperCounter>> = RefCell::new(Mutex::new(PaperCounter::default()));

    pub static PAPERS: RefCell<StableBTreeMap<PaperId, PaperDao<UserPrincipal>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))),
        )
    );

    pub static PAPER_TITLES: RefCell<HashMap<PaperId, PaperTitle>> = RefCell::new(HashMap::new());

    pub static PAPER_LEAD_AUTHORS: RefCell<BTreeMap<PaperId, UserPrincipal>> = const { RefCell::new(BTreeMap::new()) };
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    use crate::log;

    // Save USER_PRINCIPALS, USER_IDS, PAPER_COUNTER, PAPER_TITLES, and PAPER_LEAD_AUTHORS to stable storage.
    let save_result = PAPER_COUNTER.with_borrow(|paper_counter| {
        USER_EXISTENCE.with_borrow(|user_existence| {
            USER_PRINCIPALS.with_borrow(|user_principals| {
                USER_IDS.with_borrow(|user_ids| {
                    PAPER_TITLES.with_borrow(|paper_titles| {
                        PAPER_LEAD_AUTHORS.with_borrow(|paper_lead_authors| {
                            // Save all structures in a tuple
                            ic_cdk::storage::stable_save((
                                *paper_counter.lock().unwrap(),
                                user_existence.clone(),
                                user_principals.clone(),
                                user_ids.clone(),
                                paper_titles.clone(),
                                paper_lead_authors.clone()
                            ))
                        })
                    })
                })
            })
        })
    });

    // Handle error gracefully, avoiding unwrap()
    match save_result {
        Ok(_) => {
            ic_cdk::print("Successfully saved PAPER_COUNTER, USER_EXISTENCE, USER_PRINCIPALS, USER_IDS, PAPER_TITLES, and PAPER_LEAD_AUTHORS to stable storage");
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
        (PaperCounter, HashSet<UserPrincipal>, HashMap<UserId, UserPrincipal>, HashMap<UserPrincipal, UserId>, HashMap<PaperId, PaperTitle>, BTreeMap<PaperId, UserPrincipal>)
    >();

    // Handle error gracefully, avoiding unwrap()
    match restore_result {
        Ok((old_paper_counter, old_user_existence, old_user_principals, old_user_ids, old_paper_titles, old_lead_authors)) => {
            // Restore PAPER_COUNTER
            PAPER_COUNTER.with_borrow_mut(|counter| {
                *counter = Mutex::new(old_paper_counter);
            });

            // Restore USER_EXISTENCE
            USER_EXISTENCE.with_borrow_mut(|map| {
                *map = old_user_existence;
            });

            // Restore USER_PRINCIPALS
            USER_PRINCIPALS.with_borrow_mut(|map| {
                *map = old_user_principals;
            });

            // Restore USER_IDS
            USER_IDS.with_borrow_mut(|map| {
                *map = old_user_ids;
            });

            // Restore PAPER_TITLES
            PAPER_TITLES.with_borrow_mut(|map| {
                *map = old_paper_titles;
            });

            // Restore LEAD_AUTHORS
            PAPER_LEAD_AUTHORS.with_borrow_mut(|map| {
                *map = old_lead_authors;
            });

            ic_cdk::print("Successfully restored PAPER_COUNTER, USER_EXISTENCE, USER_PRINCIPALS, USER_IDS, PAPER_TITLES, and PAPER_LEAD_AUTHORS from stable storage");
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

            USER_EXISTENCE.with_borrow_mut(|map| {
                map.clear();
            });

            PAPER_TITLES.with_borrow_mut(|map| {
                map.clear();
            });

            PAPER_LEAD_AUTHORS.with_borrow_mut(|map| {
                map.clear();
            });

            // Restore PAPER_COUNTER
            PAPER_COUNTER.with_borrow_mut(|counter| {
                *counter = {
                    Mutex::new(
                        PAPERS.with_borrow(|papers| {
                            match papers.keys().last() {
                                Some(key) => {
                                    PaperCounter {
                                        last_generated_months: key.months(),
                                        count_in_month: key.number(),
                                    }
                                },
                                None => PaperCounter::default()
                            }
                        })
                    )
                };
            });

            // Repopulate USER_EXISTENCE with USERS
            USERS.with_borrow(|users| {
                USER_EXISTENCE.with_borrow_mut(|map| {
                    for user_id in users.keys() {
                        map.insert(user_id);
                    }
                });
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

            // Repopulate PAPER_TITLES from backup if available
            PAPERS.with_borrow(|backup| {
                PAPER_TITLES.with_borrow_mut(|map| {
                    for (paper_id, paper) in backup.iter() {
                        let model = domain::paper::entity::model::Paper::from_dao(paper, paper_id);
                        let title = model.title;
                        map.insert(paper_id, title);
                    }
                });
            });

            // Repopulate PAPER_LEAD_AUTHORS from backup if available
            PAPERS.with_borrow(|backup| {
                PAPER_LEAD_AUTHORS.with_borrow_mut(|map| {
                    for (paper_id, paper) in backup.iter() {
                        let model = domain::paper::entity::model::Paper::from_dao(paper, paper_id);
                        let lead_author = model.lead_author;
                        map.insert(paper_id, lead_author);
                    }
                });
            });

            ic_cdk::print("Restored PAPER_COUNTER, USER_EXISTENCE, USER_PRINCIPALS, USER_IDS, PAPER_TITLES, PAPER_LEAD_AUTHORS from backups");
        }
    }
}
