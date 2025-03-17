use candid::{CandidType, Principal};
use domain::{user::{entity::dto::User, UserId}, article::{ArticleId, entity::dto::Article}};
pub use interface_derive::Request;

pub trait Request: CandidType + Clone + Send + Sync + 'static {}

impl Request for () {}
impl Request for i8 {}
impl Request for i16 {}
impl Request for i32 {}
impl Request for i64 {}
impl Request for i128 {}
impl Request for isize {}
impl Request for u8 {}
impl Request for u16 {}
impl Request for u32 {}
impl Request for u64 {}
impl Request for u128 {}
impl Request for usize {}
impl Request for String {}

impl Request for Principal {}
impl Request for UserId {}
impl Request for ArticleId {}

impl Request for User {}
impl Request for Article {}

impl<T: Request, const N: usize> Request for [T; N] {}
impl<T: Request> Request for Vec<T> {}

impl<A: Request> Request for (A,) {}
impl<A: Request, B: Request> Request for (A, B) {}
impl<A: Request, B: Request, C: Request> Request for (A, B, C) {}
impl<A: Request, B: Request, C: Request, D: Request> Request for (A, B, C, D) {}
impl<A: Request, B: Request, C: Request, D: Request, E: Request> Request for (A, B, C, D, E) {}
impl<A: Request, B: Request, C: Request, D: Request, E: Request, F: Request> Request
    for (A, B, C, D, E, F)
{
}
impl<A: Request, B: Request, C: Request, D: Request, E: Request, F: Request, G: Request> Request
    for (A, B, C, D, E, F, G)
{
}
impl<
        A: Request,
        B: Request,
        C: Request,
        D: Request,
        E: Request,
        F: Request,
        G: Request,
        H: Request,
    > Request for (A, B, C, D, E, F, G, H)
{
}
impl<
        A: Request,
        B: Request,
        C: Request,
        D: Request,
        E: Request,
        F: Request,
        G: Request,
        H: Request,
        I: Request,
    > Request for (A, B, C, D, E, F, G, H, I)
{
}
impl<
        A: Request,
        B: Request,
        C: Request,
        D: Request,
        E: Request,
        F: Request,
        G: Request,
        H: Request,
        I: Request,
        J: Request,
    > Request for (A, B, C, D, E, F, G, H, I, J)
{
}
impl<
        A: Request,
        B: Request,
        C: Request,
        D: Request,
        E: Request,
        F: Request,
        G: Request,
        H: Request,
        I: Request,
        J: Request,
        K: Request,
    > Request for (A, B, C, D, E, F, G, H, I, J, K)
{
}
impl<
        A: Request,
        B: Request,
        C: Request,
        D: Request,
        E: Request,
        F: Request,
        G: Request,
        H: Request,
        I: Request,
        J: Request,
        K: Request,
        L: Request,
    > Request for (A, B, C, D, E, F, G, H, I, J, K, L)
{
}
