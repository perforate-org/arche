use crate::{GlobalState, GlobalStateStoreFields, Store};
use candid::{Decode, Encode, Principal};
use util::{
    canister_id::BACKEND,
    dfx_network::{DfxNetwork, DFX_NETWORK},
};
use ic_agent::{Agent, Identity};
use interface::{Request, Response};
use leptos::prelude::*;
use std::{sync::Arc, time::Duration, marker::PhantomData};

pub const TIMEOUT: Duration = Duration::from_secs(60 * 5);

/// Service is a struct that encapsulates the agent used to interact with the backend canister.
#[derive(Debug, Clone, Default)]
pub struct Service<T>
where
    T: Canister,
{
    /// Agent used to interact with the backend canister.
    agent: Option<Agent>,
    /// Marker for the generic type parameter.
    _marker: PhantomData<T>,
}

pub trait Canister {
    fn canister_id() -> Principal;
}

/// A struct that encapsulates the agent used to interact with the backend canister.
#[derive(Debug, Copy, Clone, Default)]
pub struct Backend {}

impl Canister for Backend {
    fn canister_id() -> Principal {
        *BACKEND
    }
}

impl<T> Service<T>
where
    T: Canister,
{
    pub async fn with(identity: Arc<dyn Identity>) -> Self {
        // Asynchronously create an agent to interact with the IC network.
        Self {
            agent: Some(create_agent(identity).await),
            _marker: PhantomData,
        }
    }

    /// Initiates a query call to the backend canister.
    ///
    /// Rq: The request type, implementing the Request trait.
    /// Rs: The response type, implementing the Response trait.
    ///
    /// The provided method is the name of the canister method to call.
    pub async fn query<Rq: Request, Rs: Response>(
        &mut self,
        method: &'static str,
        input: &Rq,
    ) -> Rs {
        if self.agent.is_none() {
            let state = expect_context::<Store<GlobalState>>();
            let identity = state.identity().get_untracked();

            self.agent = Some(create_agent(identity).await);
        }

        // Encode the input request into Candid binary format.
        let arg = Encode!(&input).expect("Failed to encode request using Candid");

        // Perform the query call to the backend canister.
        let res = self
            .agent
            .as_ref()
            .unwrap()
            .query(&T::canister_id(), method)
            .with_arg(arg)
            .await
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to query call: canister_id: {}, method: {}, error: {:?}",
                    T::canister_id(), method, e
                );
            });

        // Decode the response from the canister call using Candid.
        Decode!(&res.as_slice(), Rs).expect("Failed to decode response using Candid")
    }

    /// Initiates an update call to the backend canister.
    ///
    /// Rq: The request type, implementing the Request trait.
    /// Rs: The response type, implementing the Response trait.
    ///
    /// The provided method is the name of the canister method to call.
    pub async fn update<Rq: Request, Rs: Response>(
        &mut self,
        method: &'static str,
        input: &Rq,
    ) -> Rs {
        if self.agent.is_none() {
            let state = expect_context::<Store<GlobalState>>();
            let identity = state.identity().get_untracked();

            self.agent = Some(create_agent(identity).await);
        }

        // Encode the input request into Candid binary format.
        let arg = Encode!(&input).expect("Failed to encode request using Candid");

        // Perform the update call to the backend canister.
        let res = self
            .agent
            .as_ref()
            .unwrap()
            .update(&T::canister_id(), method)
            .with_arg(arg)
            .await
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to update call: canister_id: {}, method: {}, error: {:?}",
                    T::canister_id(), method, e
                );
            });

        // Decode the response from the canister call using Candid.
        Decode!(&res.as_slice(), Rs).expect("Failed to decode response using Candid")
    }
}

async fn create_agent(identity: Arc<dyn Identity>) -> Agent {
    let dfx_network = *DFX_NETWORK;

    let url = match dfx_network {
        DfxNetwork::Local => {
            let port = 4943;
            format!("http://127.0.0.1:{}", port)
        }
        DfxNetwork::Ic => "https://ic0.app".to_string(),
    };

    let agent = Agent::builder()
        .with_url(url)
        .with_arc_identity(identity)
        .with_ingress_expiry(TIMEOUT)
        .build()
        .unwrap();

    if dfx_network == DfxNetwork::Local {
        agent.fetch_root_key().await.unwrap();
    }

    agent
}
