use crate::file::FileSnapshot;
use crate::filter::{Filter, FilterAction};

/// Predicate and action, containing one or more filters.
///
/// Filters are evaluated in order. If any match, the action will be applied.
#[derive(Debug, Clone)]
pub struct Rule {
    pub filters: Vec<Filter>,
    pub action: FilterAction,
}

impl Rule {
    pub fn check(&self, file: &FileSnapshot) -> Option<Filter> {
        for filter in &self.filters {
            // if any filter matches,
            if filter.check(&file) {
                return Some(filter.clone());
            }
        }

        // if no filters match
        None
    }
}
