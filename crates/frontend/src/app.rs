use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes, RoutingProgress};
use leptos_router_macro::path;

mod author;
pub mod fallback;
mod home;

use author::Author;
use home::Home;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Title formatter=|text: String| {
                let text = match text.as_str() {
                    "" => text,
                    _ => format!("{} | ", text),
                };
                format!("{text}IC Hexagonal Architecture")
            }/>

            <main class="grid place-content-center p-8 w-screen min-h-screen">
                <Routes transition=true fallback=|| crate::Fallback>
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/author/:author_id") view=Author/>
                </Routes>
            </main>
        </Router>
    }
}
