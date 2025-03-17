use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes, RoutingProgress};
use leptos_router_macro::path;

mod article;
mod user;
pub mod fallback;
mod home;

#[component]
pub fn App() -> impl IntoView {
    let (is_routing, set_is_routing) = signal(false);

    view! {
        <RoutingProgress is_routing />
        <Router set_is_routing>
            <Title formatter=|text: String| {
                let text = match text.as_str() {
                    "" => text,
                    _ => format!("{} | ", text),
                };
                format!("{text}IC Hexagonal Architecture")
            }/>

            <main class="grid place-content-center p-8 w-screen min-h-screen">
                <Routes transition=true fallback=|| crate::Fallback>
                    <Route path=path!("/") view=home::Home/>
                    <Route path=path!("/article/:article_id") view=article::Article/>
                    <Route path=path!("/user/:user_id") view=user::User/>
                </Routes>
            </main>
        </Router>
    }
}
