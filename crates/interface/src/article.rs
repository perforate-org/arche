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

use crate::{Request, Response};
use candid::CandidType;
use common::{Post, PostCategory, PostId, PostStatus, UserId};
use serde::{Deserialize, Serialize};

/// Request to create a new article draft
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct CreateArticleRequest {
    pub title: String,
    pub summary: String,
    pub content: String,
    pub categories: Vec<PostCategory>,
    pub tags: Vec<String>,
    pub cover_image: Option<String>,
}

/// Response for article creation
#[derive(Response, CandidType, Serialize, Deserialize, Debug)]
pub struct CreateArticleResponse {
    pub post_id: PostId,
}

/// Request to update an existing article
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct UpdateArticleRequest {
    pub post_id: PostId,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub categories: Option<Vec<PostCategory>>,
    pub tags: Option<Vec<String>>,
    pub cover_image: Option<String>,
}

/// Response for article update
#[derive(Response, CandidType, Serialize, Deserialize, Debug)]
pub struct UpdateArticleResponse {
    pub post: Post,
}

/// Request to publish a draft article
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct PublishArticleRequest {
    pub post_id: PostId,
}

/// Response for article publication
#[derive(Response, CandidType, Serialize, Deserialize, Debug)]
pub struct PublishArticleResponse {
    pub post: Post,
}

/// Request to get an article by ID
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct GetArticleRequest {
    pub post_id: PostId,
}

/// Response containing article data
#[derive(Response, CandidType, Serialize, Deserialize, Debug)]
pub struct GetArticleResponse {
    pub post: Post,
}

/// Request to list articles with optional filters
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct ListArticlesRequest {
    pub author: Option<UserId>,
    pub category: Option<PostCategory>,
    pub status: Option<PostStatus>,
    pub tag: Option<String>,
    pub page_size: u32,
    pub page: u32,
}

/// Response containing a page of articles
#[derive(Response, CandidType, Serialize, Deserialize, Debug)]
pub struct ListArticlesResponse {
    pub posts: Vec<Post>,
    pub total: u32,
    pub page: u32,
    pub total_pages: u32,
}

/// Request to add a co-author to an article
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct AddCoAuthorRequest {
    pub post_id: PostId,
    pub co_author: UserId,
}

/// Response for co-author addition
#[derive(Response, CandidType, Serialize, Deserialize, Debug)]
pub struct AddCoAuthorResponse {
    pub post: Post,
}

/// Request to search articles
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct SearchArticlesRequest {
    pub query: String,
    pub category: Option<PostCategory>,
    pub page_size: u32,
    pub page: u32,
}

/// Response containing search results
#[derive(Response, CandidType, Serialize, Deserialize, Debug)]
pub struct SearchArticlesResponse {
    pub posts: Vec<Post>,
    pub total: u32,
    pub page: u32,
    pub total_pages: u32,
}
