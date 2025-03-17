use dotenvy_macro::dotenv;
use std::{env, sync::LazyLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DfxNetwork {
    Ic,
    Local,
}

pub static DFX_NETWORK: LazyLock<DfxNetwork> = LazyLock::new(dfx_network);

fn dfx_network() -> DfxNetwork {
    let mut dfx_network = dotenv!("DFX_NETWORK").to_string();
    if dfx_network.is_empty() {
        dfx_network = env::var("DFX_NETWORK").expect("DFX_NETWORK is must be set");
    }

    match dfx_network.as_str() {
        "local" => DfxNetwork::Local,
        "ic" => DfxNetwork::Ic,
        _ => panic!("Unknown dfx network: {}", dfx_network),
    }
}
