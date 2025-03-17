use crate::{GlobalState, GlobalStateStoreFields, Store, features::user::UserService};
use domain::user::{entity::dto::User, UserId};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{ParentRoute, Route, Routes},
    hooks::use_params,
    params::Params,
    MatchNestedRoutes,
};
use leptos_router_macro::path;

mod article;
mod article_list;

use article_list::ArticleList;

#[component]
pub fn UserRoute() -> impl MatchNestedRoutes + Clone {
    view! {
        <Route path=path!(":user_id") view=User>
        </Route>
    }
    .into_inner()
}

#[derive(Params, PartialEq, Debug, Clone)]
struct ContactParams {
    user_id: Option<String>,
}

#[component]
pub fn User() -> impl IntoView {
    let params = use_params::<ContactParams>();
    let service = expect_context::<Store<GlobalState>>().service();

    let user_id = move || {
        params.with(|params| {
            let str = params
                .as_ref()
                .map(|params| params.user_id.as_ref())
                .unwrap_or_default()
                .unwrap();

            match UserId::new(str) {
                Ok(id) => id,
                Err(err) => {
                    panic!("Invalid user ID: {}", err)
                }
            }
        })
    };

    let user = LocalResource::new(move || async move {
        service
            .get()
            .fetch_user(&user_id())
            .await
    });

    view! {
        <Suspense fallback=move || view! {
            <div class="skelton h-32 w-32" />
        }>
            {move || {
                user.read().as_deref().map(|result| {
                    match result {
                        Ok(user) => {
                            view! {
                                <UserInner user=user />
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

#[component]
fn UserInner<'a>(user: &'a User) -> impl IntoView {
    let name = user.name.to_owned();
    view! {
        <div>
            <Meta name="description" content=format!("User: {}", name) />
            <h1>{move || format!("User: {}", name)}</h1>
        </div>
    }
}
