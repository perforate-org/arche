extern crate console_error_panic_hook;

use ic_agent::identity::{AnonymousIdentity, Identity};
use leptos::prelude::*;
use leptos_meta::*;
use reactive_stores::Store;
use std::sync::Arc;

mod app;
mod components;
mod context;

use app::{fallback::Fallback, App};
use context::service::Service;

#[derive(Clone, Store)]
struct GlobalState {
    identity: Arc<dyn Identity>,
    service: Service,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            identity: Arc::new(AnonymousIdentity),
            service: Service::default(),
        }
    }
}

#[component]
fn Providers() -> impl IntoView {
    provide_meta_context();
    provide_context(Store::new(GlobalState::default()));

    view! {
        <App />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Providers /> });
}
