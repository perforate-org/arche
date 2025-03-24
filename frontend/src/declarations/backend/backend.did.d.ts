import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type Citation = { 'Url' : string } |
  { 'Paper' : PaperId } |
  { 'Other' : string };
export interface Paper {
  'ab' : string,
  'categories' : Array<PaperCategory>,
  'status' : PaperStatus,
  'title' : string,
  'updated_at' : bigint,
  'references' : Array<Citation>,
  'content' : string,
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
export interface PaperId {
  'version' : number,
  'number' : number,
  'months' : number,
}
export type PaperStatus = { 'UnderReview' : null } |
  { 'Draft' : null } |
  { 'Archived' : null } |
  { 'Published' : null };
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
  'lead_authored_papers' : Array<PaperId>,
  'co_authored_papers' : Array<PaperId>,
}
export interface _SERVICE {
  'fetch_caller' : ActorMethod<[], Result>,
  'fetch_paper' : ActorMethod<[string], Result_1>,
  'fetch_user' : ActorMethod<[string], Result>,
  'is_registered' : ActorMethod<[], boolean>,
  'register_user' : ActorMethod<[], Result_2>,
  'update_caller' : ActorMethod<[User], Result_2>,
  'user_exists_by_id' : ActorMethod<[string], Result_3>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
