use clap::Parser;
use miette::{Diagnostic, Result};
use nix::fcntl;
use nix::sys::stat::{fstat, FileStat, Mode};
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
enum Error {
    #[error("error reading input: {0:?}")]
    InputFailure(io::Error),

    #[error("failed to canonicalize path: {0:?}")]
    CanonicalizeFailed(io::Error),

    #[error("failed to probe file: {error:?}")]
    ProbeFailed { path: PathBuf, error: nix::Error },

    #[error("failed to stat file: {error:?}")]
    StatFailed { path: PathBuf, error: nix::Error },
}

/// Command line arguments.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Action subcommand.
    #[clap(subcommand)]
    action: Action,
}

/// Subcommands representing command line actions.
#[derive(Debug, clap::Subcommand)]
enum Action {
    /// Plan actions against filtered files.
    Plan,

    /// Execute planned actions.
    Do,
}

/// Probe of file state.
#[derive(Debug)]
struct FileProbe {
    /// Initial name provided when searching for file.
    input_name: String,

    /// Absolute path, returned by canonicalize() method.
    absolute_path: PathBuf,

    /// FileStat returned, used for integrity checking.
    stat: FileStat,

    /// Mimetype returned with tree_magic::from_filepath.
    mimetype: String,
}

/// Filter for files.
#[derive(Debug)]
enum Filter {
    /// Filter against filename using a regular expression.
    /// This matches against the canonicalized absolute path.
    Regex(&'static str),

    /// Filter against filenames with a mimetype.
    Mimetype(&'static str),
}

/// Actions to take against filtered files.
#[derive(Debug)]
enum FilterAction {
    /// Move a matched file into a destination directory.
    Move { dest: &'static str },

    /// Copy a matched file into a destination directory.
    Copy { dest: &'static str },
}

/// Predicate, containing one or more filters.
#[derive(Debug)]
struct Rule {
    filters: Vec<Filter>,
    action: FilterAction,
}

/// Probe a potential path to a file provided by the user.
fn probe_file(input_name: &str) -> Result<FileProbe> {
    let absolute_path = match PathBuf::from(input_name).canonicalize() {
        Ok(path) => path,
        Err(e) => Err(Error::CanonicalizeFailed(e))?,
    };

    let fd = match fcntl::open(&absolute_path, fcntl::OFlag::O_RDONLY, Mode::S_IRUSR) {
        Ok(fd) => fd,
        Err(error) => Err(Error::ProbeFailed {
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

/// Read stdin, probe files.
fn main() -> Result<()> {
    // TODO: parameterize behavior on subcommand
    let _args = Args::parse();

    let _rules: Vec<Rule> = vec![Rule {
        filters: vec![
            Filter::Mimetype("image/png"),
            Filter::Mimetype("image/jpeg"),
        ],
        action: FilterAction::Move { dest: "images" },
    }];

    let mut probes: HashMap<String, Result<FileProbe>> = HashMap::new();
    let stdin = io::stdin();

    for line in stdin.lines() {
        let text = match line {
            Ok(text) => text,
            Err(e) => Err(Error::InputFailure(e))?,
        };
        probes.insert(text.to_string(), probe_file(&text));
    }

    dbg!(probes);

    Ok(())
}

// design decisions
// - running `plan` cannot modify files
// - running `do` will not modify files that have changed since the `plan`
// - use abstract types to represent actions, which can be serialized + type-checked
// - do not search for files, operate on lists of files over stdin
// - canonicalize to absolute paths for all operations
// - invalid destination paths fail the entire plan
// - run fstat against files before considering a file for potential actions
// - errors resolving files are reported, partial plans are still generated
//
// tests
// - TODO: loading config
// - TODO: match against files
// - TODO: provide invalid files
// - TODO: provide invalid destination directory
// - TODO: symlink functionality
//
// design questions
// - TODO: how should directories be handled? ignore them?
// - TODO: when should duplicate files on input be resolved?
// - TODO: is special behavior needed for symlinks?
// - TODO: should fstat be asynchronous?
