use std::str::FromStr;
use crate::{GlobalState, GlobalStateStoreFields, Store, features::article::ArticleService};
use domain::ArticleId;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    hooks::use_params,
    params::Params,
};

#[derive(Params, PartialEq, Debug, Clone)]
struct ContactParams {
    article_id: Option<String>,
}

#[component]
pub fn Article() -> impl IntoView {
    let params = use_params::<ContactParams>();

    let article_id = move || {
        params.with(|params| {
            let str = params
                .as_ref()
                .map(|params| params.article_id.as_ref())
                .unwrap_or_default()
                .unwrap();

            match ArticleId::from_str(str) {
                Ok(id) => id,
                Err(err) => {
                    panic!("Invalid article ID: {}", err)
                }
            }
        })
    };

    view! {
        <div class="grid grid-cols-5 auto-rows-min gap-4 w-screen max-w-[72rem] min-h-screen p-8">
            <ArticleInner id=article_id() />
        </div>
    }
}

#[component]
pub fn ArticleInner(id: ArticleId) -> impl IntoView {
    let service = expect_context::<Store<GlobalState>>().service();

    let article = LocalResource::new(move || async move {
        service
            .get()
            .fetch_article(&id)
            .await
    });

    view!(
        <article class="col-span-4 grid grid-cols-5 gap-4 border border-[0.5px] border-gray-500 rounded-lg py-4 px-8">
            <Suspense fallback=move || view! {
                <div class="skelton h-32 w-32" />
            }>
                {move || {
                    article.read().as_deref().map(|result| {
                        match result {
                            Ok(article) => {
                                view! {
                                    <div class="col-span-5 h-auto">
                                        <h1 class="text-2xl font-bold">{article.title.to_string()}</h1>
                                        <a class="text-gray-600" href=format!("/user/{}", article.lead_author.0)>
                                            {article.lead_author.1.to_string()}
                                        </a>
                                        <p class="text-gray-600">{article.summary.to_string()}</p>
                                    </div>
                                    <div class="col-span-5">
                                        <h2 class="text-xl font-bold">Content</h2>
                                        <p class="text-gray-600">PDF</p>
                                        <p class="text-gray-600">Typst</p>
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
        </article>
    )
}
