use candid::{CandidType, Principal};
use common::{PostId, UserId};
pub use interface_derive::Response;
use serde::{Deserialize, Serialize};

pub trait Response:
    CandidType + Send + Sync + Serialize + for<'de> Deserialize<'de> + 'static
{
}

impl Response for () {}
impl Response for i8 {}
impl Response for i16 {}
impl Response for i32 {}
impl Response for i64 {}
impl Response for i128 {}
impl Response for isize {}
impl Response for u8 {}
impl Response for u16 {}
impl Response for u32 {}
impl Response for u64 {}
impl Response for u128 {}
impl Response for usize {}
impl Response for String {}

impl Response for Principal {}
impl Response for UserId {}
impl Response for PostId {}

impl<T: Response> Response for [T; 0] {}
impl<T: Response> Response for [T; 1] {}
impl<T: Response> Response for [T; 2] {}
impl<T: Response> Response for [T; 3] {}
impl<T: Response> Response for [T; 4] {}
impl<T: Response> Response for [T; 5] {}
impl<T: Response> Response for [T; 6] {}
impl<T: Response> Response for [T; 7] {}
impl<T: Response> Response for [T; 8] {}
impl<T: Response> Response for [T; 9] {}
impl<T: Response> Response for [T; 10] {}
impl<T: Response> Response for [T; 11] {}
impl<T: Response> Response for [T; 12] {}
impl<T: Response> Response for [T; 13] {}
impl<T: Response> Response for [T; 14] {}
impl<T: Response> Response for [T; 15] {}
impl<T: Response> Response for [T; 16] {}
impl<T: Response> Response for [T; 17] {}
impl<T: Response> Response for [T; 18] {}
impl<T: Response> Response for [T; 19] {}
impl<T: Response> Response for [T; 20] {}

impl<T: Response> Response for Vec<T> {}

impl<T: Response> Response for Option<T> {}
impl<T: Response, E: Response> Response for Result<T, E> {}

impl<A: Response> Response for (A,) {}
impl<A: Response, B: Response> Response for (A, B) {}
impl<A: Response, B: Response, C: Response> Response for (A, B, C) {}
impl<A: Response, B: Response, C: Response, D: Response> Response for (A, B, C, D) {}
impl<A: Response, B: Response, C: Response, D: Response, E: Response> Response for (A, B, C, D, E) {}
impl<A: Response, B: Response, C: Response, D: Response, E: Response, F: Response> Response
    for (A, B, C, D, E, F)
{
}
impl<A: Response, B: Response, C: Response, D: Response, E: Response, F: Response, G: Response>
    Response for (A, B, C, D, E, F, G)
{
}
impl<
        A: Response,
        B: Response,
        C: Response,
        D: Response,
        E: Response,
        F: Response,
        G: Response,
        H: Response,
    > Response for (A, B, C, D, E, F, G, H)
{
}
impl<
        A: Response,
        B: Response,
        C: Response,
        D: Response,
        E: Response,
        F: Response,
        G: Response,
        H: Response,
        I: Response,
    > Response for (A, B, C, D, E, F, G, H, I)
{
}
impl<
        A: Response,
        B: Response,
        C: Response,
        D: Response,
        E: Response,
        F: Response,
        G: Response,
        H: Response,
        I: Response,
        J: Response,
    > Response for (A, B, C, D, E, F, G, H, I, J)
{
}
impl<
        A: Response,
        B: Response,
        C: Response,
        D: Response,
        E: Response,
        F: Response,
        G: Response,
        H: Response,
        I: Response,
        J: Response,
        K: Response,
    > Response for (A, B, C, D, E, F, G, H, I, J, K)
{
}
impl<
        A: Response,
        B: Response,
        C: Response,
        D: Response,
        E: Response,
        F: Response,
        G: Response,
        H: Response,
        I: Response,
        J: Response,
        K: Response,
        L: Response,
    > Response for (A, B, C, D, E, F, G, H, I, J, K, L)
{
}
