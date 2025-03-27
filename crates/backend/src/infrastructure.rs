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
use ic_stable_structures::{StableBTreeMap, StableLog};
use no_panic::no_panic;
use serde::{Serialize, Deserialize};
use std::{cell::RefCell, collections::{HashMap, HashSet, BTreeMap}, sync::Mutex};

mod memory;
pub mod paper;
pub mod user;
mod pre_upgrade;
mod post_upgrade;

use memory::*;
use pre_upgrade::pre_upgrade as pre_upgrade_inner;
use post_upgrade::post_upgrade as post_upgrade_inner;

#[derive(Serialize, Deserialize)]
pub struct State {
    user_existence: HashSet<UserPrincipal>,
    user_principals: HashMap<UserId, UserPrincipal>,
    user_ids: HashMap<UserPrincipal, UserId>,
    user_names: HashMap<UserPrincipal, UserName>,
    paper_counter: Mutex<PaperCounter>,
    paper_titles: HashMap<PaperId, PaperTitle>,
    paper_lead_authors: BTreeMap<PaperId, UserPrincipal>,
    #[serde(skip, default = "init_stable_log")]
    pub log: StableLog<Log, Memory, Memory>,
    #[serde(skip, default = "init_users")]
    users: StableBTreeMap<UserPrincipal, UserDao, Memory>,
    #[serde(skip, default = "init_papers")]
    papers: StableBTreeMap<PaperId, PaperDao<UserPrincipal>, Memory>,
}

impl Default for State {
    fn default() -> Self {
        State {
            user_existence: HashSet::new(),
            user_principals: HashMap::new(),
            user_ids: HashMap::new(),
            user_names: HashMap::new(),
            paper_counter: Mutex::new(PaperCounter::default()),
            paper_titles: HashMap::new(),
            paper_lead_authors: BTreeMap::new(),
            log: init_stable_log(),
            users: init_users(),
            papers: init_papers(),
        }
    }
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());
}

#[ic_cdk::pre_upgrade]
#[no_panic]
fn pre_upgrade() {
    pre_upgrade_inner()
}

#[ic_cdk::post_upgrade]
#[no_panic]
fn post_upgrade() {
    post_upgrade_inner()
}
