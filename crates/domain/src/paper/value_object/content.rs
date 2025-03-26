use candid::CandidType;
use serde::{Serialize, Deserialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct PaperContents {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub pdf: Option<ContentFileSource>,
}

impl PaperContents {
    pub fn new(text: Option<String>, pdf: Option<ContentFileSource>) -> Self {
        Self { text, pdf }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ContentFileSource {
    Raw(RawFile),
    Http(String),
}

impl Default for ContentFileSource {
    fn default() -> Self {
        ContentFileSource::Raw(RawFile::default())
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct RawFile {
    pub name: String,
    pub content: Vec<u8>,
}

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for PaperContents {
        fn to_bytes(&self) -> Cow<[u8]> {
            Cow::Owned(candid::encode_one(self).expect("Failed to encode PaperContent"))
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            candid::decode_one(&bytes).expect("Failed to decode PaperContent")
        }

        const BOUND: Bound = Bound::Unbounded;
    }
}
