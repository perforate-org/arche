pub mod category;
pub mod citation;
pub mod content;
pub mod id;
pub mod number;
pub mod status;
pub mod summary;
pub mod title;
pub mod version;

pub use category::PaperCategory;
pub use citation::Citation;
pub use content::{PaperContent, ContentFormat, ContentSource};
pub use id::{PaperId, PaperIdError};
pub use status::PaperStatus;
pub use summary::PaperSummary;
pub use title::{PaperTitle, PaperTitleError};
