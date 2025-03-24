export const idlFactory = ({ IDL }) => {
  const PaperId = IDL.Record({
    'version' : IDL.Nat16,
    'number' : IDL.Nat32,
    'months' : IDL.Nat16,
  });
  const User = IDL.Record({
    'id' : IDL.Opt(IDL.Text),
    'name' : IDL.Text,
    'lead_authored_papers' : IDL.Vec(PaperId),
    'co_authored_papers' : IDL.Vec(PaperId),
  });
  const Result = IDL.Variant({ 'Ok' : User, 'Err' : IDL.Text });
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
  const Citation = IDL.Variant({
    'Url' : IDL.Text,
    'Paper' : PaperId,
    'Other' : IDL.Text,
  });
  const Paper = IDL.Record({
    'ab' : IDL.Text,
    'categories' : IDL.Vec(PaperCategory),
    'status' : PaperStatus,
    'title' : IDL.Text,
    'updated_at' : IDL.Nat64,
    'references' : IDL.Vec(Citation),
    'content' : IDL.Text,
    'cover_image' : IDL.Opt(IDL.Text),
    'tags' : IDL.Vec(IDL.Text),
    'lead_author' : IDL.Tuple(IDL.Text, IDL.Text),
    'created_at' : IDL.Nat64,
    'co_authors' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'citations' : IDL.Vec(Citation),
  });
  const Result_1 = IDL.Variant({ 'Ok' : Paper, 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  return IDL.Service({
    'fetch_caller' : IDL.Func([], [Result], ['query']),
    'fetch_paper' : IDL.Func([IDL.Text], [Result_1], ['query']),
    'fetch_user' : IDL.Func([IDL.Text], [Result], ['query']),
    'is_registered' : IDL.Func([], [IDL.Bool], ['query']),
    'register_user' : IDL.Func([], [Result_2], []),
    'update_caller' : IDL.Func([User], [Result_2], []),
    'user_exists_by_id' : IDL.Func([IDL.Text], [Result_3], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
