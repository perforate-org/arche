use ic_agent::{identity::AnonymousIdentity, Identity};
use ic_auth_client::{AuthClient, AuthClientLoginOptions};
use leptos::{prelude::*, leptos_dom, task::spawn_local};
use leptos_dom::logging::console_warn;
use common::util::{canister_id::internet_identity, dfx_network::{DfxNetwork, dfx_network}};
use std::sync::Arc;
use leptos::web_sys::Url;

/// Component that provides the AuthClient to the children components
#[component]
pub fn AuthClientProvider(children: Children) -> impl IntoView {
    let auth_client: Option<AuthClient> = None;
    let (auth_client, set_auth_client) = arc_signal(auth_client);
    let (identity, set_identity) = signal::<Option<Arc<dyn Identity>>>(None);

    spawn_local(async move {
        set_auth_client.set(Some(
            AuthClient::builder()
                .on_idle(|| {
                    spawn_local(async move {
                        logout().await.unwrap();
                        window().location().reload().unwrap();
                    });
                })
                .idle_timeout(20 * 60 * 1000) // 20 minutes
                .capture_scroll(true)
                .build()
                .await
                .unwrap(),
        ));
        set_identity.set(
            auth_client.get_untracked().map(|auth_client| auth_client.identity())
        )
    });

    provide_context(identity);

    children()
}

fn auth_client() -> Option<AuthClient> {
    let auth_client = use_context::<ArcReadSignal<Option<AuthClient>>>()?;
    auth_client.get_untracked()
}

pub fn get_identity() -> Arc<dyn Identity> {
    let identity = use_context::<ReadSignal<Option<Arc<dyn Identity>>>>();
    identity.get_untracked().flatten().unwrap_or_else(|| Arc::new(AnonymousIdentity))
}

pub fn login() -> Result<(), AuthClientError> {
    let dfx_network = dfx_network();

    let identity_provider = match dfx_network {
        DfxNetwork::Local => Some({
            let port = 4943;
            let canister_id = internet_identity();
            Url::new(&format!("http://{}.localhost:{}", canister_id, port)).unwrap()
        }),
        DfxNetwork::Ic => None,
    };

    let on_success = |_| {
        window().location().reload().unwrap();
    };
    let on_error = |e| {
        if let Some(e) = e {
            console_warn(&format!("Failed to login: {:?}", e));
        } else {
            console_warn("Failed to login");
        }
    };

    let options = match identity_provider {
        Some(identity_provider) => AuthClientLoginOptions::builder().identity_provider(identity_provider),
        None => AuthClientLoginOptions::builder(),
    };
    let options = options
        .on_success(on_success)
        .on_error(on_error)
        .build();

    auth_client()?.login_with_options(options);

    Ok(())
}

pub async fn logout() -> Result<(), AuthClientError> {
    auth_client()?.logout(Some(window().location())).await;
    Ok(())
}
