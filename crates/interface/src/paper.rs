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
    paper::{entity::dto::Paper, value_object::{PaperCategory, PaperStatus}},
    PaperId, UserId,
};

/// Request to create a new paper draft
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct CreatePaperRequest {
    pub title: String,
    pub ab: String,
    pub content: String,
    pub categories: Vec<PaperCategory>,
    pub tags: Vec<String>,
    pub cover_image: Option<String>,
}

/// Response for paper creation
#[derive(Response, CandidType, Serialize, Deserialize, Debug)]
pub struct CreatePaperResponse {
    pub paper_id: PaperId,
}

/// Request to update an existing paper
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct UpdatePaperRequest {
    pub paper_id: PaperId,
    pub title: Option<String>,
    pub ab: Option<String>,
    pub content: Option<String>,
    pub categories: Option<Vec<PaperCategory>>,
    pub tags: Option<Vec<String>>,
    pub cover_image: Option<String>,
}

/// Response for paper update
#[derive(Response, CandidType, Deserialize, Debug)]
pub struct UpdatePaperResponse {
    pub paper: Paper,
}

/// Request to publish a draft paper
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct PublishPaperRequest {
    pub paper_id: PaperId,
}

/// Response for paper publication
#[derive(Response, CandidType, Deserialize, Debug)]
pub struct PublishPaperResponse {
    pub paper: Paper,
}

/// Request to get an paper by ID
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct GetPaperRequest {
    pub paper_id: PaperId,
}

/// Response containing paper data
#[derive(Response, CandidType, Deserialize, Debug)]
pub struct GetPaperResponse {
    pub paper: Paper,
}

/// Request to list papers with optional filters
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct ListPapersRequest {
    pub author: Option<UserId>,
    pub category: Option<PaperCategory>,
    pub status: Option<PaperStatus>,
    pub tag: Option<String>,
    pub page_size: u32,
    pub page: u32,
}

/// Response containing a page of papers
#[derive(Response, CandidType, Deserialize, Debug)]
pub struct ListPapersResponse {
    pub papers: Vec<Paper>,
    pub total: u32,
    pub page: u32,
    pub total_pages: u32,
}

/// Request to add a co-author to an paper
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct AddCoAuthorRequest {
    pub paper_id: PaperId,
    pub co_author: UserId,
}

/// Response for co-author addition
#[derive(Response, CandidType, Deserialize, Debug)]
pub struct AddCoAuthorResponse {
    pub paper: Paper,
}

/// Request to search papers
#[derive(Request, Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct SearchPapersRequest {
    pub query: String,
    pub category: Option<PaperCategory>,
    pub page_size: u32,
    pub page: u32,
}

/// Response containing search results
#[derive(Response, CandidType, Deserialize, Debug)]
pub struct SearchPapersResponse {
    pub papers: Vec<Paper>,
    pub total: u32,
    pub page: u32,
    pub total_pages: u32,
}
