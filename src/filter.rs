use crate::file::FileSnapshot;

/// Filter for files.
///
/// Filters can be checked against probed files.
#[derive(Debug, Clone)]
pub enum Filter {
    /// Filter against filename using a regular expression.
    /// This matches against the canonicalized absolute path.
    Regex(&'static str),
    /// Filter against filenames with a mimetype.
    Mimetype(&'static str),
}

impl Filter {
    pub fn check(&self, file: &FileSnapshot) -> bool {
        match self {
            Filter::Regex(String) => false,
            Filter::Mimetype(mimetype) => file.mimetype == *mimetype,
        }
    }
}

/// Actions to take against filtered files.
/// TODO: make sure paths are valid
#[derive(Debug, Clone)]
pub enum FilterAction {
    /// Move a matched file into a destination directory.
    Move { dest: &'static str },
    /// Copy a matched file into a destination directory.
    Copy { dest: &'static str },
}

impl FilterAction {
    pub fn apply(&self, target: FileSnapshot) {
        match self {
            FilterAction::Move { .. } => todo!(),
            FilterAction::Copy { .. } => todo!(),
        }
    }
}
