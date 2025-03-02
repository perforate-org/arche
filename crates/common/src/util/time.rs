/// Returns the current time in nanoseconds.
///
/// The implementation varies based on the target platform:
/// - Uses `wasm_timer` when compiled with "wasm-js" feature
/// - Uses Internet Computer's time API when compiled with "canister" feature
/// - Uses standard library's SystemTime otherwise
pub fn now() -> u64 {
    #[cfg(feature = "wasm-js")]
    {
        now_wasm_js()
    }

    #[cfg(all(feature = "canister", not(feature = "wasm-js")))]
    {
        now_canister()
    }

    #[cfg(not(any(feature = "canister", feature = "wasm-js")))]
    {
        now_std()
    }
}

#[cfg(feature = "wasm-js")]
fn now_wasm_js() -> u64 {
    use wasm_timer::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64
}

#[cfg(all(feature = "canister", not(feature = "wasm-js")))]
fn now_canister() -> u64 {
    ic_cdk::api::time()
}

#[cfg(not(any(feature = "canister", feature = "wasm-js")))]
fn now_std() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64
}
