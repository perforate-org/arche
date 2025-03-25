use candid::CandidType;
use serde::{Serialize, Deserialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct PaperContent {
    pub content_format: ContentFormat,
    pub content_source: ContentSource,
}

impl PaperContent {
    pub fn new(content_format: ContentFormat, content_source: ContentSource) -> Self {
        Self {
            content_format,
            content_source,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Copy, Debug, Default)]
pub enum ContentFormat {
    #[default]
    Text,
    Tex,
    Latex,
    Typst,
    Satysfi,
    Pdf,
    Markdown,
    Html,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ContentSource {
    Raw(Vec<u8>),
    Http(String),
    Arweave(String),
    Ipfs(String),
}

impl Default for ContentSource {
    fn default() -> Self {
        ContentSource::Raw(Vec::new())
    }
}

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for PaperContent {
        fn to_bytes(&self) -> Cow<[u8]> {
            Cow::Owned(candid::encode_one(self).expect("Failed to encode PaperContent"))
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            candid::decode_one(&bytes).expect("Failed to decode PaperContent")
        }

        const BOUND: Bound = Bound::Unbounded;
    }
}
