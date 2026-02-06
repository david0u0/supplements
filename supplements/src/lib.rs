pub mod completion;
pub mod error;
pub mod history;
pub mod id;
pub mod info;

mod generate;
mod utils;
pub use completion::{Completion, Shell};
pub use generate::generate;
pub use history::History;
pub use utils::*;

pub(crate) mod arg_context;
pub(crate) mod parsed_flag;

pub type Result<T = ()> = std::result::Result<T, error::Error>;
