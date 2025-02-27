use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text=""/>
        <div class="flex flex-col items-center justify-center">
            <h1 class="text-4xl font-bold">Welcome to Leptos!</h1>
            <p class="text-xl">This is a simple example of a Leptos application.</p>
        </div>
    }
}
