use candid::CandidType;
use derive_more::{AsRef, Display};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error returned when creating an invalid paper title
#[derive(Error, Debug, Clone, PartialEq)]
pub enum PaperTitleError {
    #[error("paper title cannot be empty")]
    Empty,

    #[error("paper title exceeds maximum length of {0} characters")]
    TooLong(usize),
}

/// A strongly typed wrapper for paper titles
///
/// Paper titles must not be empty and must be less than 100 characters.
#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, AsRef, Display)]
#[as_ref(forward)]
pub struct PaperTitle(String);

impl PaperTitle {
    /// Maximum length for paper titles
    pub const MAX_LENGTH: usize = 2000;

    /// Creates a new PaperTitle, validating that it's not empty and doesn't exceed the maximum length
    ///
    /// # Examples
    /// ```
    /// # use crate::{PaperTitle, PaperTitleError};
    /// let title = PaperTitle::new("My Blog Paper").unwrap();
    /// assert_eq!(title.as_str(), "My Blog Paper");
    ///
    /// let empty_result = PaperTitle::new("");
    /// assert!(empty_result.is_err());
    /// ```
    pub fn new(title: impl Into<String>) -> Result<Self, PaperTitleError> {
        let title_string: String = title.into();

        if title_string.trim().is_empty() {
            return Err(PaperTitleError::Empty);
        }

        if title_string.chars().count() > Self::MAX_LENGTH {
            return Err(PaperTitleError::TooLong(Self::MAX_LENGTH));
        }

        Ok(Self(title_string))
    }

    /// Creates a new PaperTitle without validation
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
    /// # use crate::PaperTitle;
    /// # let title = unsafe { PaperTitle::new_unchecked("Hello") };
    /// assert_eq!(title.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the title is empty
    ///
    /// Note: This should always return false for valid PaperTitle instances
    /// since empty titles are rejected by the constructor.
    ///
    /// # Examples
    /// ```
    /// # use crate::PaperTitle;
    /// # let title = unsafe { PaperTitle::new_unchecked("Example") };
    /// assert!(!title.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a reference to the title as a string
    ///
    /// # Examples
    /// ```
    /// # use crate::PaperTitle;
    /// # let title = unsafe { PaperTitle::new_unchecked("Example") };
    /// assert_eq!(title.as_str(), "Example");
    /// ```
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Trims whitespace from the title and returns a new PaperTitle
    ///
    /// # Examples
    /// ```
    /// # use crate::PaperTitle;
    /// # let title = unsafe { PaperTitle::new_unchecked("  Hello World  ") };
    /// assert_eq!(title.trim().as_str(), "Hello World");
    /// ```
    pub fn trim(&self) -> Self {
        // We use unsafe because we know the trimmed title won't be empty
        // since our constructor prevents empty titles from being created
        unsafe { Self::new_unchecked(self.0.trim()) }
    }
}

// TryFrom implementation for String
impl TryFrom<String> for PaperTitle {
    type Error = PaperTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// TryFrom implementation for &String
impl TryFrom<&String> for PaperTitle {
    type Error = PaperTitleError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// TryFrom implementation for &str
impl TryFrom<&str> for PaperTitle {
    type Error = PaperTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// From PaperTitle to String
impl From<PaperTitle> for String {
    fn from(title: PaperTitle) -> Self {
        title.0
    }
}

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for PaperTitle {
        fn to_bytes(&self) -> Cow<[u8]> {
            self.0.to_bytes()
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            // This is safe because we're assuming data from stable storage
            // was validated when it was first created
            unsafe { PaperTitle::new_unchecked(String::from_bytes(bytes)) }
        }

        const BOUND: Bound = String::BOUND;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let title = PaperTitle::new("Test Paper").unwrap();
        assert_eq!(title.as_str(), "Test Paper");
    }

    #[test]
    fn test_new_empty() {
        let result = PaperTitle::new("");
        assert_eq!(result, Err(PaperTitleError::Empty));

        let result = PaperTitle::new("   ");
        assert_eq!(result, Err(PaperTitleError::Empty));
    }

    #[test]
    fn test_new_too_long() {
        let long_title = "a".repeat(PaperTitle::MAX_LENGTH + 1);
        let result = PaperTitle::new(long_title);
        assert_eq!(result, Err(PaperTitleError::TooLong(PaperTitle::MAX_LENGTH)));
    }

    #[test]
    fn test_len() {
        let title = PaperTitle::new("Test Paper").unwrap();
        assert_eq!(title.len(), 9);
    }

    #[test]
    fn test_is_empty() {
        // This can only be tested with unsafe since we can't create empty titles normally
        let title = unsafe { PaperTitle::new_unchecked("Not Empty") };
        assert!(!title.is_empty());
    }

    #[test]
    fn test_display() {
        let title = PaperTitle::new("Test Paper").unwrap();
        assert_eq!(format!("{}", title), "Test Paper");
    }

    #[test]
    fn test_try_from() {
        let title_string = String::from("Test Paper");
        let title: PaperTitle = title_string.clone().try_into().unwrap();
        assert_eq!(title.as_str(), "Test Paper");

        let back_to_string: String = title.into();
        assert_eq!(back_to_string, title_string);
    }

    #[test]
    fn test_trim() {
        let title = PaperTitle::new("  Padded Title  ").unwrap();
        assert_eq!(title.trim().as_str(), "Padded Title");
    }
}
