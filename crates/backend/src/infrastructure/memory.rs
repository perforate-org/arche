use domain::{
    paper::{
        entity::dao::PaperDao,
        value_object::PaperId,
    }, user::{
        entity::dao::UserDao,
        value_object::UserPrincipal,
    },
};
use crate::log::Log;
use ic_cdk::api::print;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, StableLog,
};
use std::cell::RefCell;

const UPGRADES: MemoryId = MemoryId::new(0);
const LOG_INDEX: MemoryId = MemoryId::new(1);
const LOG_DATA: MemoryId = MemoryId::new(2);
const USERS: MemoryId = MemoryId::new(3);
const PAPERS: MemoryId = MemoryId::new(4);

pub(super) type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    pub(super) static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

pub(super) fn get_upgrades_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(UPGRADES))
}

pub(super) fn init_stable_log() -> StableLog<Log, Memory, Memory> {
    let result = StableLog::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(LOG_INDEX)),
        MEMORY_MANAGER.with(|m| m.borrow().get(LOG_DATA)),
    );

    match result {
        Ok(log) => log,
        Err(e) => {
            print(format!("Failed to initialize log: {}", e));
            ic_cdk::trap("Critical error: Failed to initialize stable log");
        }
    }
}

pub(super) fn init_users() -> StableBTreeMap<UserPrincipal, UserDao, Memory> {
    StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(USERS)),
    )
}

pub(super) fn init_papers() -> StableBTreeMap<PaperId, PaperDao<UserPrincipal>, Memory> {
    StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(PAPERS)),
    )
}
