// Copyright 2025 Inomoto, Yota
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use candid::Principal;
use common::{Post, PostCategory, PostId, PostStatus, PostTitle, UserId};
use ic_stable_structures::{
    btreemap::BTreeMap,
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};
use interface::article::{
    AddCoAuthorRequest, AddCoAuthorResponse, CreateArticleRequest, CreateArticleResponse,
    GetArticleRequest, GetArticleResponse, ListArticlesRequest, ListArticlesResponse,
    PublishArticleRequest, PublishArticleResponse, SearchArticlesRequest, SearchArticlesResponse,
    UpdateArticleRequest, UpdateArticleResponse,
};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ARTICLES: RefCell<BTreeMap<PostId, Post, Memory>> = RefCell::new(
        BTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );
}

pub fn create_article(caller: Principal, request: CreateArticleRequest) -> CreateArticleResponse {
    let user_id = UserId::from(caller);
    let post_id = PostId::generate();
    let title = PostTitle::new(&request.title).expect("Invalid title");

    let article = Post::new_draft(
        user_id,
        title,
        request.summary,
        request.content,
        request.categories,
        request.tags,
    );

    ARTICLES.with(|articles| {
        articles.borrow_mut().insert(post_id.clone(), article);
    });

    CreateArticleResponse { post_id }
}

pub fn update_article(caller: Principal, request: UpdateArticleRequest) -> UpdateArticleResponse {
    let user_id = UserId::from(caller);
    ARTICLES.with(|articles| {
        let mut articles = articles.borrow_mut();
        let mut article = articles.get(&request.post_id).expect("Article not found");

        // Verify caller is author or co-author
        assert!(
            article.primary_author == user_id || article.co_authors.contains(&user_id),
            "Not authorized to update this article"
        );

        if let Some(title) = request.title {
            article.title = PostTitle::new(&title).expect("Invalid title");
        }
        if let Some(summary) = request.summary {
            article.summary = summary;
        }
        if let Some(content) = request.content {
            article.content = content;
        }
        if let Some(categories) = request.categories {
            article.categories = categories;
        }
        if let Some(tags) = request.tags {
            article.tags = tags;
        }
        if let Some(cover_image) = request.cover_image {
            article.cover_image = Some(cover_image);
        }

        article.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        articles.insert(request.post_id, article.clone());

        UpdateArticleResponse { post: article }
    })
}

pub fn publish_article(
    caller: Principal,
    request: PublishArticleRequest,
) -> PublishArticleResponse {
    let user_id = UserId::from(caller);
    ARTICLES.with(|articles| {
        let mut articles = articles.borrow_mut();
        let mut article = articles.get(&request.post_id).expect("Article not found");

        assert!(
            article.primary_author == user_id,
            "Only primary author can publish"
        );

        article.publish().expect("Failed to publish article");
        articles.insert(request.post_id, article.clone());

        PublishArticleResponse { post: article }
    })
}

pub fn get_article(_caller: Principal, request: GetArticleRequest) -> GetArticleResponse {
    ARTICLES.with(|articles| {
        let mut articles = articles.borrow_mut();
        let mut article = articles
            .get(&request.post_id)
            .expect("Article not found")
            .clone();

        // Increment view count for published articles
        if article.status == PostStatus::Published {
            article.increment_views();
            articles.insert(request.post_id, article.clone());
        }

        GetArticleResponse { post: article }
    })
}

pub fn list_articles(_caller: Principal, request: ListArticlesRequest) -> ListArticlesResponse {
    ARTICLES.with(|articles| {
        let articles = articles.borrow();
        let mut matching_posts: Vec<Post> = articles
            .iter()
            .filter(|(_, post)| {
                let author_match = request
                    .author
                    .as_ref()
                    .map(|author| &post.primary_author == author)
                    .unwrap_or(true);

                let category_match = request
                    .category
                    .as_ref()
                    .map(|cat| post.categories.contains(cat))
                    .unwrap_or(true);

                let status_match = request
                    .status
                    .as_ref()
                    .map(|status| &post.status == status)
                    .unwrap_or(true);

                let tag_match = request
                    .tag
                    .as_ref()
                    .map(|tag| post.tags.contains(tag))
                    .unwrap_or(true);

                author_match && category_match && status_match && tag_match
            })
            .map(|(_, post)| post.clone())
            .collect();

        // Sort by newest first
        matching_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let total = matching_posts.len() as u32;
        let start = (request.page * request.page_size) as usize;
        let end = ((request.page + 1) * request.page_size) as usize;
        let posts = matching_posts
            .into_iter()
            .skip(start)
            .take(end - start)
            .collect();
        let total_pages = (total + request.page_size - 1) / request.page_size;

        ListArticlesResponse {
            posts,
            total,
            page: request.page,
            total_pages,
        }
    })
}

pub fn add_co_author(caller: Principal, request: AddCoAuthorRequest) -> AddCoAuthorResponse {
    let user_id = UserId::from(caller);
    ARTICLES.with(|articles| {
        let mut articles = articles.borrow_mut();
        let mut article = articles.get(&request.post_id).expect("Article not found");

        assert!(
            article.primary_author == user_id,
            "Only primary author can add co-authors"
        );

        if !article.co_authors.contains(&request.co_author) {
            article.co_authors.push(request.co_author);
            articles.insert(request.post_id, article.clone());
        }

        AddCoAuthorResponse { post: article }
    })
}

pub fn search_articles(
    _caller: Principal,
    request: SearchArticlesRequest,
) -> SearchArticlesResponse {
    ARTICLES.with(|articles| {
        let articles = articles.borrow();
        let mut matching_posts: Vec<Post> = articles
            .iter()
            .filter(|(_, post)| {
                let query = request.query.to_lowercase();
                let title_match = post.title.to_string().to_lowercase().contains(&query);
                let summary_match = post.summary.to_lowercase().contains(&query);
                let content_match = post.content.to_lowercase().contains(&query);
                let tag_match = post
                    .tags
                    .iter()
                    .any(|tag| tag.to_lowercase().contains(&query));

                let category_match = request
                    .category
                    .as_ref()
                    .map(|cat| post.categories.contains(cat))
                    .unwrap_or(true);

                (title_match || summary_match || content_match || tag_match) && category_match
            })
            .map(|(_, post)| post.clone())
            .collect();

        // Sort by relevance (currently just newest first)
        matching_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let total = matching_posts.len() as u32;
        let start = (request.page * request.page_size) as usize;
        let end = ((request.page + 1) * request.page_size) as usize;
        let posts = matching_posts
            .into_iter()
            .skip(start)
            .take(end - start)
            .collect();
        let total_pages = (total + request.page_size - 1) / request.page_size;

        SearchArticlesResponse {
            posts,
            total,
            page: request.page,
            total_pages,
        }
    })
}
