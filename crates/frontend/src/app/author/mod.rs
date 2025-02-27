use common::UserId;
use crate::{Store, GlobalState, GlobalStateStoreFields};
use leptos::{html::audio, prelude::*};
use leptos_meta::*;
use leptos_router::{
    components::{ParentRoute, Route, Routes},
    hooks::use_params,
    params::Params, MatchNestedRoutes,
};
use leptos_router_macro::path;

mod post;
mod post_list;

use post_list::PostList;

#[component]
pub fn AuthorRoute() -> impl MatchNestedRoutes + Clone {
    view! {
        <Route path=path!(":author_id") view=Author>
        </Route>
    }
    .into_inner()
}

#[derive(Params, PartialEq, Debug, Clone)]
struct ContactParams {
    author_id: Option<String>,
}

#[component]
pub fn Author() -> impl IntoView {
    let params = use_params::<ContactParams>();
    let service = expect_context::<Store<GlobalState>>().service();

    let author_id = move || {
        params.with(|params| {
            let str = params
                .as_ref()
                .map(|params| params.author_id.as_ref())
                .unwrap_or_default()
                .unwrap();
            match UserId::new(str) {
                Ok(id) => id,
                Err(err) => {
                    panic!("Invalid author ID: {}", err)
                }
            }
        })
    };

    let profile = LocalResource::new(
        move || {
            async move {
                service.get().query::<UserId, Result<interface::user::UserProfileResponse, String>>(
                    "get_author_profile",
                    &author_id(),
                ).await
            }
        }
    );

    view! {
        <Suspense fallback=move || view! { <div>"Loading..."</div> }>
            {move || {
                profile.read().as_deref().map(|result| {
                    match result {
                        Ok(profile) => {
                            let name = profile.name.to_owned();
                            view! {
                                <div>
                                    <Meta name="description" content=format!("Author: {}", name) />
                                    <h1>{move || format!("Author: {}", name)}</h1>
                                </div>
                            }.into_any()
                        }
                        Err(err) => {
                            view! {
                                <div>
                                    <h1>"Error"</h1>
                                    <p>{err.to_string()}</p>
                                </div>
                            }.into_any()
                        }
                    }
                })
            }}
        </Suspense>
    }
}
