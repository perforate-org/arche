use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Fallback() -> impl IntoView {
    view! {
        <Title text="404"/>
        <p>"This page is not found."</p>
    }
}
