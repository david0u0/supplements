use std::fs;

pub mod error;
pub mod id;
pub mod info;

mod generate;
pub mod history;
mod utils;
pub use generate::generate;
pub use history::History;
pub use utils::*;

pub(crate) mod arg_context;
pub(crate) mod parsed_flag;

pub type Result<T = ()> = std::result::Result<T, error::Error>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Completion {
    pub value: String,
    pub description: String,
}
impl Completion {
    pub fn new(value: &str, description: &str) -> Self {
        Completion {
            value: value.to_owned(),
            description: description.to_owned(),
        }
    }
    pub fn files() -> Vec<Self> {
        let paths = match fs::read_dir("./") {
            Ok(paths) => paths,
            Err(err) => {
                log::warn!("error reading current directory: {:?}", err);
                return vec![];
            }
        };

        paths
            .filter_map(|p| {
                let p = match p {
                    Ok(p) => p.path(),
                    Err(err) => {
                        log::warn!("error reading current directory: {:?}", err);
                        return None;
                    }
                };
                let Some(file_name) = p.file_name() else {
                    return None;
                };
                Some(Completion::new(file_name.to_string_lossy().as_ref(), ""))
            })
            .collect()
    }
}
