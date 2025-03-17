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

use crate::{Request, Response, CandidType, Deserialize, Serialize};
use domain::{
    article::{entity::dto::Article, value_object::{ArticleCategory, ArticleStatus}},
    ArticleId, UserId,
};

/// Request to create a new article draft
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct CreateArticleRequest {
    pub title: String,
    pub summary: String,
    pub content: String,
    pub categories: Vec<ArticleCategory>,
    pub tags: Vec<String>,
    pub cover_image: Option<String>,
}

/// Response for article creation
#[derive(Response, CandidType, Serialize, Deserialize, Debug)]
pub struct CreateArticleResponse {
    pub article_id: ArticleId,
}

/// Request to update an existing article
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct UpdateArticleRequest {
    pub article_id: ArticleId,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub categories: Option<Vec<ArticleCategory>>,
    pub tags: Option<Vec<String>>,
    pub cover_image: Option<String>,
}

/// Response for article update
#[derive(Response, CandidType, Deserialize, Debug)]
pub struct UpdateArticleResponse {
    pub article: Article,
}

/// Request to publish a draft article
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct PublishArticleRequest {
    pub article_id: ArticleId,
}

/// Response for article publication
#[derive(Response, CandidType, Deserialize, Debug)]
pub struct PublishArticleResponse {
    pub article: Article,
}

/// Request to get an article by ID
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct GetArticleRequest {
    pub article_id: ArticleId,
}

/// Response containing article data
#[derive(Response, CandidType, Deserialize, Debug)]
pub struct GetArticleResponse {
    pub article: Article,
}

/// Request to list articles with optional filters
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct ListArticlesRequest {
    pub author: Option<UserId>,
    pub category: Option<ArticleCategory>,
    pub status: Option<ArticleStatus>,
    pub tag: Option<String>,
    pub page_size: u32,
    pub page: u32,
}

/// Response containing a page of articles
#[derive(Response, CandidType, Deserialize, Debug)]
pub struct ListArticlesResponse {
    pub articles: Vec<Article>,
    pub total: u32,
    pub page: u32,
    pub total_pages: u32,
}

/// Request to add a co-author to an article
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct AddCoAuthorRequest {
    pub article_id: ArticleId,
    pub co_author: UserId,
}

/// Response for co-author addition
#[derive(Response, CandidType, Deserialize, Debug)]
pub struct AddCoAuthorResponse {
    pub article: Article,
}

/// Request to search articles
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct SearchArticlesRequest {
    pub query: String,
    pub category: Option<ArticleCategory>,
    pub page_size: u32,
    pub page: u32,
}

/// Response containing search results
#[derive(Response, CandidType, Deserialize, Debug)]
pub struct SearchArticlesResponse {
    pub articles: Vec<Article>,
    pub total: u32,
    pub page: u32,
    pub total_pages: u32,
}
