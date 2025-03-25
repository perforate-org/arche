import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type Citation = { 'Url' : string } |
  { 'Paper' : PaperId } |
  { 'Other' : string };
export type ContentFormat = { 'Pdf' : null } |
  { 'Tex' : null } |
  { 'Latex' : null } |
  { 'Html' : null } |
  { 'Text' : null } |
  { 'Satysfi' : null } |
  { 'Markdown' : null } |
  { 'Typst' : null };
export type ContentSource = { 'Raw' : Uint8Array | number[] } |
  { 'Http' : string } |
  { 'Ipfs' : string } |
  { 'Arweave' : string };
export interface Paper {
  'ab' : string,
  'id' : string,
  'categories' : Array<PaperCategory>,
  'status' : PaperStatus,
  'title' : string,
  'updated_at' : bigint,
  'references' : Array<Citation>,
  'content' : PaperContent,
  'cover_image' : [] | [string],
  'tags' : Array<string>,
  'lead_author' : [string, string],
  'created_at' : bigint,
  'co_authors' : Array<[string, string]>,
  'citations' : Array<Citation>,
}
export type PaperCategory = { 'MachineLearning' : null } |
  { 'SystemDesign' : null } |
  { 'Security' : null } |
  { 'Blockchain' : null } |
  { 'DevOps' : null } |
  { 'Programming' : null } |
  { 'Other' : string };
export interface PaperContent {
  'content_format' : ContentFormat,
  'content_source' : ContentSource,
}
export interface PaperId {
  'version' : number,
  'number' : number,
  'months' : number,
}
export interface PaperIdTitle { 'id' : string, 'title' : string }
export type PaperStatus = { 'UnderReview' : null } |
  { 'Draft' : null } |
  { 'Archived' : null } |
  { 'Published' : null };
export interface PaperSummaryDto {
  'id' : string,
  'title' : string,
  'lead_author_name' : string,
  'lead_author_id' : string,
}
export type Result = { 'Ok' : User } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : Paper } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : boolean } |
  { 'Err' : string };
export interface User {
  'id' : [] | [string],
  'name' : string,
  'lead_authored_papers' : Array<PaperIdTitle>,
  'co_authored_papers' : Array<PaperIdTitle>,
}
export interface _SERVICE {
  'create_draft' : ActorMethod<[], string>,
  'fetch_all_paper_summaries' : ActorMethod<[], Array<PaperSummaryDto>>,
  'fetch_caller' : ActorMethod<[], Result>,
  'fetch_paper' : ActorMethod<[string], Result_1>,
  'fetch_paper_as_author' : ActorMethod<[string], Result_1>,
  'fetch_user' : ActorMethod<[string], Result>,
  'is_registered' : ActorMethod<[], boolean>,
  'register_user' : ActorMethod<[], Result_2>,
  'update_caller' : ActorMethod<[User], Result_2>,
  'update_paper' : ActorMethod<[Paper], Result_2>,
  'user_exists_by_id' : ActorMethod<[string], Result_3>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
