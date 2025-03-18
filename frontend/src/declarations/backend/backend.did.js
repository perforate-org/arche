export const idlFactory = ({ IDL }) => {
  const ArticleCategory = IDL.Variant({
    'MachineLearning' : IDL.Null,
    'SystemDesign' : IDL.Null,
    'Security' : IDL.Null,
    'Blockchain' : IDL.Null,
    'DevOps' : IDL.Null,
    'Programming' : IDL.Null,
    'Other' : IDL.Text,
  });
  const ArticleStatus = IDL.Variant({
    'UnderReview' : IDL.Null,
    'Draft' : IDL.Null,
    'Archived' : IDL.Null,
    'Published' : IDL.Null,
  });
  const ArticleId = IDL.Record({
    'version' : IDL.Nat16,
    'number' : IDL.Nat32,
    'months' : IDL.Nat16,
  });
  const Citation = IDL.Variant({
    'Url' : IDL.Text,
    'Article' : ArticleId,
    'Other' : IDL.Text,
  });
  const Article = IDL.Record({
    'categories' : IDL.Vec(ArticleCategory),
    'status' : ArticleStatus,
    'title' : IDL.Text,
    'updated_at' : IDL.Nat64,
    'references' : IDL.Vec(Citation),
    'content' : IDL.Text,
    'cover_image' : IDL.Opt(IDL.Text),
    'tags' : IDL.Vec(IDL.Text),
    'lead_author' : IDL.Tuple(IDL.Text, IDL.Text),
    'created_at' : IDL.Nat64,
    'summary' : IDL.Text,
    'co_authors' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'citations' : IDL.Vec(Citation),
  });
  const Result = IDL.Variant({ 'Ok' : Article, 'Err' : IDL.Text });
  const User = IDL.Record({
    'name' : IDL.Text,
    'lead_authored_articles' : IDL.Vec(ArticleId),
    'co_authored_articles' : IDL.Vec(ArticleId),
  });
  const Result_1 = IDL.Variant({ 'Ok' : User, 'Err' : IDL.Text });
  const RegisterUserRequest = IDL.Record({
    'id' : IDL.Vec(IDL.Nat8),
    'name' : IDL.Text,
  });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  return IDL.Service({
    'fetch_article' : IDL.Func([IDL.Text], [Result], ['query']),
    'fetch_user' : IDL.Func([IDL.Text], [Result_1], ['query']),
    'register_user' : IDL.Func([RegisterUserRequest], [Result_2], []),
  });
};
export const init = ({ IDL }) => { return []; };
