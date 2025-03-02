use candid::Principal;
use common::{
    post::{PostId, PostKey},
    user::{UserId, UserPrincipal},
};
use ic_cdk::{
    api::{call::CallResult, caller, print},
    storage,
};
use ic_cdk_macros::*;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, StableLog,
};
use interface::article::{
    AddCoAuthorRequest, AddCoAuthorResponse, CreateArticleRequest, CreateArticleResponse,
    GetArticleRequest, GetArticleResponse, ListArticlesRequest, ListArticlesResponse,
    PublishArticleRequest, PublishArticleResponse, SearchArticlesRequest, SearchArticlesResponse,
    UpdateArticleRequest, UpdateArticleResponse,
};
use std::{cell::RefCell, collections::HashMap};

mod log;
mod post;
mod user;
use user::{User, UserV1};

type Memory = VirtualMemory<DefaultMemoryImpl>;

// Article management functions
#[update]
fn create_article(request: CreateArticleRequest) -> CreateArticleResponse {
    post::create_article(caller(), request)
}

#[update]
fn update_article(request: UpdateArticleRequest) -> UpdateArticleResponse {
    post::update_article(caller(), request)
}

#[update]
fn publish_article(request: PublishArticleRequest) -> PublishArticleResponse {
    post::publish_article(caller(), request)
}

#[query]
fn get_article(request: GetArticleRequest) -> GetArticleResponse {
    post::get_article(caller(), request)
}

#[query]
fn list_articles(request: ListArticlesRequest) -> ListArticlesResponse {
    post::list_articles(caller(), request)
}

#[update]
fn add_co_author(request: AddCoAuthorRequest) -> AddCoAuthorResponse {
    post::add_co_author(caller(), request)
}

#[query]
fn search_articles(request: SearchArticlesRequest) -> SearchArticlesResponse {
    post::search_articles(caller(), request)
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static LOG: RefCell<StableLog<log::Log, Memory, Memory>> = RefCell::new({
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

    pub static USER_IDS: RefCell<StableBTreeMap<UserPrincipal, UserId, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
        )
    );

    pub static USERS: RefCell<StableBTreeMap<UserPrincipal, User, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))),
        )
    );
}

fn get_user_principal(user_id: &UserId) -> Option<UserPrincipal> {
    USER_PRINCIPALS.with(|map| map.borrow().get(user_id).copied())
}

fn change_user_id(new_id: UserId) -> Result<UserId, String> {
    let caller = caller().into();
    let old_id = USER_IDS
        .with(|map| map.borrow().get(&caller))
        .ok_or_else(|| format!("User (Principal: {}) does not exist", caller))?;

    // First check if the new ID already exists
    if USER_PRINCIPALS.with(|map| map.borrow().contains_key(&new_id)) {
        return Err(format!("User (ID: {}) already exists", new_id));
    }

    // Update USER_PRINCIPALS using Entry API
    USER_PRINCIPALS.with(|map| {
        let mut users = map.borrow_mut();
        users.insert(new_id, caller);
        users.remove(&old_id);
    });

    // Update USER_PRINCIPALS_BACKUP
    USER_PRINCIPALS_BACKUP.with(|map| {
        let mut users = map.borrow_mut();
        users.insert(new_id, caller);
        users.remove(&old_id);
    });

    // Update USER_IDS
    USER_IDS.with(|map| {
        map.borrow_mut().insert(caller, new_id);
    });

    Ok(old_id)
}

#[init]
fn init() {
    let anonymous_principal = Principal::anonymous().into();
    let anonymous_id = UserId::new("anonymous").unwrap();

    // Initialize the USER_PRINCIPALS map with the anonymous user principal.
    USER_PRINCIPALS.with(|map| {
        map.borrow_mut().insert(anonymous_id, anonymous_principal);
    });

    // Initialize the USER_PRINCIPALS_BACKUP map with the anonymous user principal.
    USER_PRINCIPALS_BACKUP.with(|map| {
        map.borrow_mut().insert(anonymous_id, anonymous_principal);
    });

    // Initialize the USER_IDS map with the anonymous user ID.
    USER_IDS.with(|map| {
        map.borrow_mut().insert(anonymous_principal, anonymous_id);
    });

    // Initialize the USERS map with the anonymous user.
    USERS.with(|map| {
        map.borrow_mut().insert(
            anonymous_principal,
            User::V1(UserV1 {
                name: "Anonymous Author".to_string(),
            }),
        );
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    // Save the USER_PRINCIPALS map to stable storage.
    // Handle error gracefully, avoiding unwrap()
    match USER_PRINCIPALS.with(|map| {
        // Use correct clone syntax for borrowed reference
        storage::stable_save((map,))
    }) {
        Ok(_) => (),
        Err(e) => {
            ic_cdk::print(format!("Failed to save USER_PRINCIPALS: {:?}", e));
            let _ = log::error(
                "Failed to save USER_PRINCIPALS",
                &format!("{:?}", e),
                "pre_upgrade",
            );
        }
    }
}

#[post_upgrade]
fn post_upgrade() {
    // Load the USER_PRINCIPALS map from stable storage.
    // Handle error gracefully, avoiding unwrap()
    match storage::stable_restore::<(HashMap<UserId, UserPrincipal>,)>() {
        Ok((old_user_principals,)) => {
            USER_PRINCIPALS.with(|map| {
                *map.borrow_mut() = old_user_principals;
            });
            ic_cdk::print("Successfully restored USER_PRINCIPALS from stable storage");
        }
        Err(e) => {
            ic_cdk::print(format!("Failed to restore USER_PRINCIPALS: {:?}", e));
            // Initialize with empty map to ensure system can still function
            USER_PRINCIPALS.with(|map| {
                map.borrow_mut().clear();
            });

            // Repopulate from backup if available
            USER_PRINCIPALS_BACKUP.with(|backup| {
                let backup_map = backup.borrow();
                USER_PRINCIPALS.with(|map| {
                    let mut map = map.borrow_mut();
                    for (user_id, principal) in backup_map.iter() {
                        map.insert(user_id, principal);
                    }
                });
            });
            ic_cdk::print("Restored USER_PRINCIPALS from backup");
        }
    }
}

#[query]
fn get_author_profile(user_id: UserId) -> Result<interface::user::UserProfileResponse, String> {
    // Retrieve the principal associated with the provided user_id.
    let principal = get_user_principal(&user_id)
        .ok_or_else(|| format!("User (ID: {}) does not exist", user_id))?;

    // Attempt to obtain the user's record from the USERS map.
    let maybe_name = USERS.with(|map| {
        map.borrow()
            .get(&principal)
            .map(|user_entry| match user_entry {
                User::V1(user_data) => user_data.name.clone(),
            })
    });

    // Check if the user's name was found; if not, return an error.
    let name = maybe_name.ok_or_else(|| format!("User (ID: {}) does not exist", user_id))?;

    Ok(interface::user::UserProfileResponse { name })
}

#[query]
fn get_author_profile_with_str(
    user_id: String,
) -> Result<interface::user::UserProfileResponse, String> {
    // Convert the user_id string to an UserId.
    let user_id = UserId::new(&user_id).map_err(|_| format!("Invalid user ID: {}", user_id))?;

    // Call the get_user_profile function with the converted UserId.
    get_author_profile(user_id)
}

ic_cdk::export_candid!();
