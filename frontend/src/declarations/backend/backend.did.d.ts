import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Article {
  'categories' : Array<ArticleCategory>,
  'status' : ArticleStatus,
  'title' : string,
  'updated_at' : bigint,
  'references' : Array<Citation>,
  'content' : string,
  'cover_image' : [] | [string],
  'tags' : Array<string>,
  'lead_author' : [string, string],
  'created_at' : bigint,
  'summary' : string,
  'co_authors' : Array<[string, string]>,
  'citations' : Array<Citation>,
}
export type ArticleCategory = { 'MachineLearning' : null } |
  { 'SystemDesign' : null } |
  { 'Security' : null } |
  { 'Blockchain' : null } |
  { 'DevOps' : null } |
  { 'Programming' : null } |
  { 'Other' : string };
export interface ArticleId {
  'version' : number,
  'number' : number,
  'months' : number,
}
export type ArticleStatus = { 'UnderReview' : null } |
  { 'Draft' : null } |
  { 'Archived' : null } |
  { 'Published' : null };
export type Citation = { 'Url' : string } |
  { 'Article' : ArticleId } |
  { 'Other' : string };
export interface RegisterUserRequest {
  'id' : Uint8Array | number[],
  'name' : string,
}
export type Result = { 'Ok' : Article } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : User } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : null } |
  { 'Err' : string };
export interface User {
  'name' : string,
  'lead_authored_articles' : Array<ArticleId>,
  'co_authored_articles' : Array<ArticleId>,
}
export interface _SERVICE {
  'fetch_article' : ActorMethod<[string], Result>,
  'fetch_user' : ActorMethod<[string], Result_1>,
  'register_user' : ActorMethod<[RegisterUserRequest], Result_2>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
