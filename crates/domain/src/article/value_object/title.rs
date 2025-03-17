use candid::CandidType;
use derive_more::{AsRef, Display};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error returned when creating an invalid article title
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ArticleTitleError {
    #[error("article title cannot be empty")]
    Empty,

    #[error("article title exceeds maximum length of {0} characters")]
    TooLong(usize),
}

/// A strongly typed wrapper for article titles
///
/// Article titles must not be empty and must be less than 100 characters.
#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, AsRef, Display)]
#[as_ref(forward)]
pub struct ArticleTitle(String);

impl ArticleTitle {
    /// Maximum length for article titles
    pub const MAX_LENGTH: usize = 2000;

    /// Creates a new ArticleTitle, validating that it's not empty and doesn't exceed the maximum length
    ///
    /// # Examples
    /// ```
    /// # use crate::{ArticleTitle, ArticleTitleError};
    /// let title = ArticleTitle::new("My Blog Article").unwrap();
    /// assert_eq!(title.as_str(), "My Blog Article");
    ///
    /// let empty_result = ArticleTitle::new("");
    /// assert!(empty_result.is_err());
    /// ```
    pub fn new(title: impl Into<String>) -> Result<Self, ArticleTitleError> {
        let title_string: String = title.into();

        if title_string.trim().is_empty() {
            return Err(ArticleTitleError::Empty);
        }

        if title_string.chars().count() > Self::MAX_LENGTH {
            return Err(ArticleTitleError::TooLong(Self::MAX_LENGTH));
        }

        Ok(Self(title_string))
    }

    /// Creates a new ArticleTitle without validation
    ///
    /// # Safety
    ///
    /// This function should only be used when you're certain the title meets
    /// the requirements (non-empty and under maximum length).
    pub unsafe fn new_unchecked(title: impl Into<String>) -> Self {
        Self(title.into())
    }

    /// Returns the length of the title in bytes
    ///
    /// # Examples
    /// ```
    /// # use crate::ArticleTitle;
    /// # let title = unsafe { ArticleTitle::new_unchecked("Hello") };
    /// assert_eq!(title.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the title is empty
    ///
    /// Note: This should always return false for valid ArticleTitle instances
    /// since empty titles are rejected by the constructor.
    ///
    /// # Examples
    /// ```
    /// # use crate::ArticleTitle;
    /// # let title = unsafe { ArticleTitle::new_unchecked("Example") };
    /// assert!(!title.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a reference to the title as a string
    ///
    /// # Examples
    /// ```
    /// # use crate::ArticleTitle;
    /// # let title = unsafe { ArticleTitle::new_unchecked("Example") };
    /// assert_eq!(title.as_str(), "Example");
    /// ```
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Trims whitespace from the title and returns a new ArticleTitle
    ///
    /// # Examples
    /// ```
    /// # use crate::ArticleTitle;
    /// # let title = unsafe { ArticleTitle::new_unchecked("  Hello World  ") };
    /// assert_eq!(title.trim().as_str(), "Hello World");
    /// ```
    pub fn trim(&self) -> Self {
        // We use unsafe because we know the trimmed title won't be empty
        // since our constructor prevents empty titles from being created
        unsafe { Self::new_unchecked(self.0.trim()) }
    }
}

// TryFrom implementation for String
impl TryFrom<String> for ArticleTitle {
    type Error = ArticleTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// TryFrom implementation for &String
impl TryFrom<&String> for ArticleTitle {
    type Error = ArticleTitleError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// TryFrom implementation for &str
impl TryFrom<&str> for ArticleTitle {
    type Error = ArticleTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// From ArticleTitle to String
impl From<ArticleTitle> for String {
    fn from(title: ArticleTitle) -> Self {
        title.0
    }
}

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for ArticleTitle {
        fn to_bytes(&self) -> Cow<[u8]> {
            self.0.to_bytes()
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            // This is safe because we're assuming data from stable storage
            // was validated when it was first created
            unsafe { ArticleTitle::new_unchecked(String::from_bytes(bytes)) }
        }

        const BOUND: Bound = String::BOUND;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let title = ArticleTitle::new("Test Article").unwrap();
        assert_eq!(title.as_str(), "Test Article");
    }

    #[test]
    fn test_new_empty() {
        let result = ArticleTitle::new("");
        assert_eq!(result, Err(ArticleTitleError::Empty));

        let result = ArticleTitle::new("   ");
        assert_eq!(result, Err(ArticleTitleError::Empty));
    }

    #[test]
    fn test_new_too_long() {
        let long_title = "a".repeat(ArticleTitle::MAX_LENGTH + 1);
        let result = ArticleTitle::new(long_title);
        assert_eq!(result, Err(ArticleTitleError::TooLong(ArticleTitle::MAX_LENGTH)));
    }

    #[test]
    fn test_len() {
        let title = ArticleTitle::new("Test Article").unwrap();
        assert_eq!(title.len(), 9);
    }

    #[test]
    fn test_is_empty() {
        // This can only be tested with unsafe since we can't create empty titles normally
        let title = unsafe { ArticleTitle::new_unchecked("Not Empty") };
        assert!(!title.is_empty());
    }

    #[test]
    fn test_display() {
        let title = ArticleTitle::new("Test Article").unwrap();
        assert_eq!(format!("{}", title), "Test Article");
    }

    #[test]
    fn test_try_from() {
        let title_string = String::from("Test Article");
        let title: ArticleTitle = title_string.clone().try_into().unwrap();
        assert_eq!(title.as_str(), "Test Article");

        let back_to_string: String = title.into();
        assert_eq!(back_to_string, title_string);
    }

    #[test]
    fn test_trim() {
        let title = ArticleTitle::new("  Padded Title  ").unwrap();
        assert_eq!(title.trim().as_str(), "Padded Title");
    }
}
