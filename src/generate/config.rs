use super::Trace;
use crate::error::GenerateError;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Config {
    ignore: HashMap<Vec<String>, bool>,
}

impl Config {
    pub fn ignore(mut self, ids: &[&str]) -> Self {
        self.ignore
            .insert(ids.iter().map(|x| x.to_string()).collect(), false);
        self
    }

    pub(crate) fn is_ignored(&mut self, prev: &[Trace], id: &str) -> bool {
        let mut key: Vec<_> = prev.iter().map(|t| t.cmd_id.to_string()).collect();
        key.push(id.to_string());
        if let Some(t) = self.ignore.get_mut(&key) {
            *t = true;
            true
        } else {
            false
        }
    }
    pub(crate) fn unprocessed_ignore(&self) -> impl Iterator<Item = &[String]> {
        self.ignore.iter().filter_map(|(key, processed)| {
            if *processed {
                None
            } else {
                Some(key.as_slice())
            }
        })
    }

    pub(crate) fn check_unprocessed_config(&self) -> Result<(), GenerateError> {
        let mut it = self.unprocessed_ignore().peekable();
        if it.peek().is_none() {
            return Ok(());
        }

        Err(GenerateError::UnprocessedConfigObj(
            it.map(|x| x.to_vec()).collect(),
        ))
    }
}
