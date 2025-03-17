use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use thiserror::Error;

const UNIX_EPOCH_YEAR: u16 = 1970;

/// Error types for ArticleId operations
#[derive(Error, Debug)]
pub enum ArticleIdError {
    #[error("Invalid ArticleId format: expected YYYY-MM-number[-vV], got '{0}'")]
    InvalidFormat(String),

    #[error("Invalid year format: expected YYYY, got '{0}'")]
    InvalidYearFormat(String),

    #[error("Invalid year: could not parse '{0}'")]
    InvalidYearParse(String),

    #[error("Invalid month format: expected MM, got '{0}'")]
    InvalidMonthFormat(String),

    #[error("Invalid month: must be between 1 and 12, got {0}")]
    InvalidMonthRange(u16),

    #[error("Invalid month: could not parse '{0}'")]
    InvalidMonthParse(String),

    #[error("Invalid number: could not parse '{0}'")]
    InvalidNumberParse(String),

    #[error("Invalid version format: expected v followed by number, got '{0}'")]
    InvalidVersionFormat(String),

    #[error("Invalid version: must be >= 1")]
    InvalidVersionValue,

    #[error("Invalid version: could not parse '{0}'")]
    InvalidVersionParse(String),

    #[error("Invalid version in ArticleId: version must be greater than zero, got {0}")]
    ZeroVersion(u16),
}

/// Represents a unique identifier for articles in the system.
///
/// # Format
/// `YYYY-MM-number-vV` where:
///- `YYYY`: Year (e.g., 2025)
/// - `MM`: Month (01-12)
/// - `number`: Sequential identifier within the month (starts at 1, increments with each new article)
/// - `vV`: Version number (only displayed when version > 1)
///
/// # Examples
/// - `2025-04-0001` - First article created in April 2025, version 1 (default)
/// - `2025-04-0002-v2` - Second article created in April 2025, version 2 (after an update)
///
/// # Components
/// - `months`: Internal representation as months since Unix epoch (January 1970)
/// - `number`: The unique sequence number within the month
/// - `version`: The revision version, starting at 1 for new articles
#[derive(
    CandidType,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Debug,
    Hash,
    Eq,
    PartialEq,
)]
pub struct ArticleId {
    months: u16,
    number: u32,
    version: u16,
}

impl Ord for ArticleId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.months
            .cmp(&other.months)
            .then(self.number.cmp(&other.number))
            .then(self.version.cmp(&other.version))
    }
}

impl PartialOrd for ArticleId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for ArticleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let year = self.months / 12 + UNIX_EPOCH_YEAR;
        let month = self.months % 12 + 1; // Add 1 to get months 1-12 instead of 0-11

        // Use write! directly instead of creating an intermediate string
        write!(f, "{:04}-{:02}-{:04}", year, month, self.number)?;

        // Only append version if it's not 1 (default version)
        if self.version > 1 {
            write!(f, "-v{}", self.version)
        } else {
            Ok(())
        }
    }
}

impl FromStr for ArticleId {
    type Err = ArticleIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Pattern: YYYY-MM-number[-vV]
        let parts: Vec<&str> = s.split('-').collect();

        if parts.len() < 3 || parts.len() > 4 {
            return Err(ArticleIdError::InvalidFormat(s.to_string()));
        }

        // Parse YYYY
        let year_part = parts[0];
        if year_part.len() != 4 {
            return Err(ArticleIdError::InvalidYearFormat(year_part.to_string()));
        }

        let year = match year_part.parse::<u16>() {
            Ok(y) => y - UNIX_EPOCH_YEAR,
            Err(_) => return Err(ArticleIdError::InvalidYearParse(year_part.to_string())),
        };

        // Parse MM
        let month_part = parts[1];
        if month_part.len() != 2 {
            return Err(ArticleIdError::InvalidMonthFormat(month_part.to_string()));
        }

        let month = match month_part.parse::<u16>() {
            Ok(m) if (1..=12).contains(&m) => m - 1,
            Ok(m) => return Err(ArticleIdError::InvalidMonthRange(m)),
            Err(_) => return Err(ArticleIdError::InvalidMonthParse(parts[1].to_string())),
        };

        let months = year * 12 + month;

        // Parse number
        let number = match parts[2].parse::<u32>() {
            Ok(n) => n,
            Err(_) => return Err(ArticleIdError::InvalidNumberParse(parts[2].to_string())),
        };

        // Parse version if present
        let version = if parts.len() == 4 {
            let version_part = parts[3];
            if !version_part.starts_with('v') {
                return Err(ArticleIdError::InvalidVersionFormat(version_part.to_string()));
            }

            match version_part[1..].parse::<u16>() {
                Ok(v) if v >= 1 => v,
                Ok(_) => return Err(ArticleIdError::InvalidVersionValue),
                Err(_) => return Err(ArticleIdError::InvalidVersionParse(version_part[1..].to_string())),
            }
        } else {
            1
        };

        // We can directly create the struct here as we've already validated the version
        Ok(Self {
            months,
            number,
            version,
        })
    }
}

impl ArticleId {
    /// Creates a new ArticleId
    pub fn new(months: u16, number: u32, version: u16) -> Result<Self, ArticleIdError> {
        // Validate version is not zero
        if version == 0 {
            return Err(ArticleIdError::ZeroVersion(version));
        }

        Ok(Self {
            months,
            number,
            version,
        })
    }

    pub fn months(&self) -> u16 {
        self.months
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn version(&self) -> u16 {
        self.version
    }
}

#[cfg(feature = "ic-stable")]
mod ic_stable {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;

    impl Storable for ArticleId {
        fn to_bytes(&self) -> Cow<[u8]> {
            Cow::Owned([self.months.to_bytes(), self.number.to_bytes(), self.version.to_bytes()].concat())
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            ArticleId {
                months: u16::from_bytes(Cow::Borrowed(&bytes[0..2])),
                number: u32::from_bytes(Cow::Borrowed(&bytes[2..6])),
                version: u16::from_bytes(Cow::Borrowed(&bytes[6..8])),
            }
        }

        const BOUND: Bound = Bound::Bounded {
            max_size: 8,
            is_fixed_size: true,
        };
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_id_creation() {
        // Valid creation
        let id = ArticleId::new(600, 42, 1).unwrap();
        assert_eq!(id.months, 600);
        assert_eq!(id.number, 42);
        assert_eq!(id.version, 1);

        // Invalid creation (version = 0)
        let err = ArticleId::new(600, 42, 0).unwrap_err();
        match err {
            ArticleIdError::ZeroVersion(v) => assert_eq!(v, 0),
            _ => panic!("Expected ZeroVersion error"),
        }
    }

    #[test]
    fn test_article_id_display() {
        // Test normal display (version 1)
        let id = ArticleId { months: 660, number: 7, version: 1 };
        assert_eq!(id.to_string(), "2025-01-0007");

        // Test with version > 1
        let id = ArticleId { months: 661, number: 42, version: 3 };
        assert_eq!(id.to_string(), "2025-02-0042-v3");
    }

    #[test]
    fn test_article_id_from_str() {
        // Valid parsing
        let id: ArticleId = "2023-05-0012".parse().unwrap();
        assert_eq!(id.months, (2023 - UNIX_EPOCH_YEAR) * 12 + 5 - 1);
        assert_eq!(id.number, 12);
        assert_eq!(id.version, 1);

        // Valid parsing with version
        let id: ArticleId = "2023-05-0012-v3".parse().unwrap();
        assert_eq!(id.months, (2023 - UNIX_EPOCH_YEAR) * 12 + 5 - 1);
        assert_eq!(id.number, 12);
        assert_eq!(id.version, 3);
    }

    #[test]
    fn test_article_id_from_str_errors() {
        // Test various error cases
        let cases = [
            ("202-05-0012", ArticleIdError::InvalidYearFormat("202".to_string())),
            ("abcd-05-0012", ArticleIdError::InvalidYearParse("abcd".to_string())),
            ("2023-5-0012", ArticleIdError::InvalidMonthFormat("5".to_string())),
            ("2023-13-0012", ArticleIdError::InvalidMonthRange(13)),
            ("2023-ab-0012", ArticleIdError::InvalidMonthParse("ab".to_string())),
            ("2023-05-abcd", ArticleIdError::InvalidNumberParse("abcd".to_string())),
            ("2023-05-0012-x3", ArticleIdError::InvalidVersionFormat("x3".to_string())),
            ("2023-05-0012-v0", ArticleIdError::InvalidVersionValue),
            ("2023-05-0012-vx", ArticleIdError::InvalidVersionParse("x".to_string())),
            ("2023-05", ArticleIdError::InvalidFormat("2023-05".to_string())),
        ];

        for (input, expected_err) in cases {
            let result: Result<ArticleId, ArticleIdError> = input.parse();
            assert!(result.is_err(), "Expected error for input: {}", input);

            // This only checks the error variant, not the specific message
            match result.unwrap_err() {
                ref e if std::mem::discriminant(e) == std::mem::discriminant(&expected_err) => {},
                e => panic!("For input '{}', expected {:?} but got {:?}", input, expected_err, e),
            }
        }
    }

    #[test]
    fn test_article_id_ordering() {
        let id1 = ArticleId { months: 660, number: 1, version: 1 };
        let id2 = ArticleId { months: 660, number: 2, version: 1 };
        let id3 = ArticleId { months: 660, number: 2, version: 2 };
        let id4 = ArticleId { months: 661, number: 1, version: 1 };

        assert!(id1 < id2);
        assert!(id2 < id3);
        assert!(id3 < id4);

        // Test equality
        assert_eq!(id1, id1);
        assert_ne!(id1, id2);

        // Sort test
        let mut ids = vec![&id4, &id3, &id1, &id2];
        ids.sort();
        assert_eq!(ids, vec![&id1, &id2, &id3, &id4]);
    }
}
