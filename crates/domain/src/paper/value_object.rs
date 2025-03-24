pub mod category;
pub mod citation;
pub mod id;
pub mod number;
pub mod status;
pub mod title;
pub mod version;

pub use category::PaperCategory;
pub use citation::Citation;
pub use id::{PaperId, PaperIdError};
pub use status::PaperStatus;
pub use title::{PaperTitle, PaperTitleError};
