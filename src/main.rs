use clap::Parser;
use nix::fcntl;
use nix::sys::stat::{fstat, FileStat, Mode};
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

type Result<A> = std::result::Result<A, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("error reading input: {0:?}")]
    InputFailure(io::Error),
    #[error("failed to canonicalize path: {0:?}")]
    CanonicalizeFailed(io::Error),
    #[error("failed to open file: {error:?}")]
    OpenFailed { path: PathBuf, error: nix::Error },
    #[error("failed to stat file: {error:?}")]
    StatFailed { path: PathBuf, error: nix::Error },
}

/// Command line arguments.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Action subcommand.
    #[clap(subcommand)]
    pub action: Action,
}

/// Subcommands representing command line actions.
#[derive(Debug, clap::Subcommand)]
pub enum Action {
    /// Plan actions against filtered files.
    Plan,
    /// Execute planned actions.
    Do,
}

/// Probe of file state.
#[derive(Debug, Clone)]
pub struct FileProbe {
    /// Initial name provided when searching for file.
    pub input_name: String,
    /// Absolute path, returned by canonicalize() method.
    pub absolute_path: PathBuf,
    /// FileStat returned, used for integrity checking.
    pub stat: FileStat,
    /// Mimetype returned with tree_magic::from_filepath.
    pub mimetype: String,
}

/// Filter for files.
#[derive(Debug, Clone)]
pub enum Filter {
    /// Filter against filename using a regular expression.
    /// This matches against the canonicalized absolute path.
    Regex(&'static str),
    /// Filter against filenames with a mimetype.
    Mimetype(&'static str),
}

impl Filter {
    pub fn check(&self, file: &FileProbe) -> bool {
        match self {
            Filter::Regex(String) => false,
            Filter::Mimetype(mimetype) => file.mimetype == *mimetype,
        }
    }
}

/// Actions to take against filtered files.
#[derive(Debug, Clone)]
enum FilterAction {
    /// Move a matched file into a destination directory.
    Move { dest: String },
    /// Copy a matched file into a destination directory.
    Copy { dest: String },
}

impl FilterAction {
    fn apply(&self, target: FileProbe) {
        match self {
            FilterAction::Move { .. } => todo!(),
            FilterAction::Copy { .. } => todo!(),
        }
    }
}

/// Predicate, containing one or more filters.
#[derive(Debug, Clone)]
struct Rule {
    filters: Vec<Filter>,
    action: FilterAction,
}

impl Rule {
    pub fn check(&self, file: &FileProbe) -> Option<Filter> {
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

/// Probe a potential path to a file provided by the user.
fn probe_file(input_name: &str) -> Result<FileProbe> {
    let absolute_path = match PathBuf::from(input_name).canonicalize() {
        Ok(path) => path,
        Err(e) => Err(Error::CanonicalizeFailed(e))?,
    };

    let fd = match fcntl::open(&absolute_path, fcntl::OFlag::O_RDONLY, Mode::S_IRUSR) {
        Ok(fd) => fd,
        Err(error) => Err(Error::OpenFailed {
            path: absolute_path.clone(),
            error,
        })?,
    };

    let stat = match fstat(fd) {
        Ok(stat) => stat,
        Err(error) => Err(Error::StatFailed {
            path: absolute_path.clone(),
            error,
        })?,
    };

    let mimetype = tree_magic::from_filepath(&absolute_path);

    Ok(FileProbe {
        input_name: input_name.into(),
        absolute_path,
        stat,
        mimetype,
    })
}

/// Did the FileProbe match any Filters?
#[derive(Debug)]
enum Match {
    /// Yes! We have an action to apply.
    Yes {
        rule: Rule,
        filter: Filter,
        file: FileProbe,
    },
    /// No, this file does not match any Rules.
    No { file: FileProbe },
}

/// Read stdin, probe files.
fn main() -> Result<()> {
    let args = Args::parse();

    // define filters and actions
    let rules: Vec<Rule> = vec![Rule {
        filters: vec![
            Filter::Mimetype("image/png"),
            Filter::Mimetype("image/jpeg"),
            Filter::Mimetype("image/gif"),
            Filter::Mimetype("image/bmp"),
            Filter::Mimetype("image/svg+xml"),
        ],
        action: FilterAction::Move {
            dest: "images".into(),
        },
    }];

    // get input and probe files
    let mut probes: HashMap<String, Result<FileProbe>> = HashMap::new();
    let stdin = io::stdin();
    for line in stdin.lines() {
        let text = line.map_err(Error::InputFailure)?;
        probes.insert(text.to_string(), probe_file(&text));
    }

    // check success and failure
    let (success, failure): (Vec<_>, Vec<_>) =
        probes.iter().partition(|(_, result)| result.is_ok());
    println!(
        "probed {0}/{1} files successfully",
        success.len(),
        probes.len()
    );

    // exit if there were any failures probing file (prompt the user to resolve)
    if failure.len() != 0 {
        dbg!(failure);
    }

    // start matching against our rules
    // TODO: make this a function
    for result in probes.values() {
        if let Ok(file) = result {
            for rule in &rules {
                if let Some(filter) = rule.check(file) {
                    // match
                    dbg!((rule, filter, file));
                }
            }
        }
    }

    println!("done");
    Ok(())
}

// kdl
// ---
// meta {
//   version "1.0"
//   created "Tue, 1 Jul 2003 10:52:37 +0200"
// }
//
// files {
//   file "/proc" {
//     mimetype "inode/directory"
//     absolute "/proc"
//     timestamp {
//       atime "15698164562"
//       ctime "15698164562"
//       mtime "15698164562"
//     }
//   }
// }

// tests
// - TODO: match against files
// - TODO: provide invalid files
// - TODO: provide invalid destination directory
// - TODO: symlink functionality
