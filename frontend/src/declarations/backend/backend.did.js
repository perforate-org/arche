export const idlFactory = ({ IDL }) => {
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const PaperSummaryDto = IDL.Record({
    'id' : IDL.Text,
    'title' : IDL.Text,
    'lead_author_name' : IDL.Text,
    'lead_author_id' : IDL.Text,
  });
  const PaperIdTitle = IDL.Record({ 'id' : IDL.Text, 'title' : IDL.Text });
  const User = IDL.Record({
    'id' : IDL.Opt(IDL.Text),
    'name' : IDL.Text,
    'lead_authored_papers' : IDL.Vec(PaperIdTitle),
    'co_authored_papers' : IDL.Vec(PaperIdTitle),
  });
  const Result_1 = IDL.Variant({ 'Ok' : User, 'Err' : IDL.Text });
  const PaperCategory = IDL.Variant({
    'MachineLearning' : IDL.Null,
    'SystemDesign' : IDL.Null,
    'Security' : IDL.Null,
    'Blockchain' : IDL.Null,
    'DevOps' : IDL.Null,
    'Programming' : IDL.Null,
    'Other' : IDL.Text,
  });
  const PaperStatus = IDL.Variant({
    'UnderReview' : IDL.Null,
    'Draft' : IDL.Null,
    'Archived' : IDL.Null,
    'Published' : IDL.Null,
  });
  const PaperId = IDL.Record({
    'version' : IDL.Nat16,
    'number' : IDL.Nat32,
    'months' : IDL.Nat16,
  });
  const Citation = IDL.Variant({
    'Url' : IDL.Text,
    'Paper' : PaperId,
    'Other' : IDL.Text,
  });
  const RawFile = IDL.Record({
    'content' : IDL.Vec(IDL.Nat8),
    'name' : IDL.Text,
  });
  const ContentFileSource = IDL.Variant({ 'Raw' : RawFile, 'Http' : IDL.Text });
  const PaperContents = IDL.Record({
    'pdf' : IDL.Opt(ContentFileSource),
    'text' : IDL.Opt(IDL.Text),
  });
  const Paper = IDL.Record({
    'ab' : IDL.Text,
    'id' : IDL.Text,
    'categories' : IDL.Vec(PaperCategory),
    'status' : PaperStatus,
    'title' : IDL.Text,
    'updated_at' : IDL.Nat64,
    'references' : IDL.Vec(Citation),
    'content' : PaperContents,
    'cover_image' : IDL.Opt(IDL.Text),
    'tags' : IDL.Vec(IDL.Text),
    'lead_author' : IDL.Tuple(IDL.Text, IDL.Text),
    'created_at' : IDL.Nat64,
    'co_authors' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'citations' : IDL.Vec(Citation),
  });
  const Result_2 = IDL.Variant({ 'Ok' : Paper, 'Err' : IDL.Text });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  return IDL.Service({
    'create_draft' : IDL.Func([], [IDL.Text], []),
    'delete_paper' : IDL.Func([IDL.Text], [Result], []),
    'fetch_all_paper_summaries' : IDL.Func(
        [],
        [IDL.Vec(PaperSummaryDto)],
        ['query'],
      ),
    'fetch_caller' : IDL.Func([], [Result_1], ['query']),
    'fetch_paper' : IDL.Func([IDL.Text], [Result_2], ['query']),
    'fetch_paper_as_author' : IDL.Func([IDL.Text], [Result_2], ['query']),
    'fetch_user' : IDL.Func([IDL.Text], [Result_1], ['query']),
    'is_registered' : IDL.Func([], [IDL.Bool], ['query']),
    'register_user' : IDL.Func([], [Result], []),
    'update_caller' : IDL.Func([User], [Result], []),
    'update_paper' : IDL.Func([Paper], [Result], []),
    'user_exists_by_id' : IDL.Func([IDL.Text], [Result_3], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
