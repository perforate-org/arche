use candid::Principal;
use dotenvy_macro::dotenv;
use std::{env, sync::LazyLock};

pub static BACKEND: LazyLock<Principal> = LazyLock::new(backend);

pub static INTERNET_IDENTITY: LazyLock<Principal> = LazyLock::new(internet_identity);

fn backend() -> Principal {
    let mut canister_id = dotenv!("CANISTER_ID_BACKEND").to_string();
    if canister_id.is_empty() {
        canister_id = env::var("CANISTER_ID_BACKEND").expect("CANISTER_ID_BACKEND is must be set");
    }

    parse_principal(canister_id)
}

fn internet_identity() -> Principal {
    let mut canister_id = dotenv!("CANISTER_ID_INTERNET_IDENTITY").to_string();
    if canister_id.is_empty() {
        canister_id = env::var("CANISTER_ID_INTERNET_IDENTITY")
            .expect("CANISTER_ID_INTERNET_IDENTITY is must be set");
    }

    parse_principal(canister_id)
}

fn parse_principal(canister_id: String) -> Principal {
    Principal::from_text(canister_id).expect("Failed to parse canister principal")
}
