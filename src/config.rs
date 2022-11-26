use crate::error::Ganzu;
use crate::file::FileSnapshot;
use crate::filter::Filter;
use crate::rule::Rule;

#[derive(Debug, Clone)]
pub struct Config {
    pub rules: Vec<Rule>,
}

impl Config {
    pub fn check(&self, file: &FileSnapshot) -> Option<Filter> {
        todo!();
    }

    pub fn validate(&self) -> Ganzu<ConfigValidation> {
        todo!();
    }
}

pub struct ConfigValidation {}
