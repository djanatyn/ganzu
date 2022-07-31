#![feature(stdin_forwarders)]

use clap::Parser;
use nix::fcntl;
use nix::sys::stat::{fstat, FileStat, Mode};
use std::io;
use std::path::PathBuf;

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
fn probe_file(input_name: &str) -> io::Result<FileProbe> {
    let absolute_path = PathBuf::from(input_name).canonicalize()?;
    let fd = fcntl::open(&absolute_path, fcntl::OFlag::O_RDONLY, Mode::S_IRUSR)
        .expect("could not open file");
    let stat = fstat(fd).expect("could not stat file");
    let mimetype = tree_magic::from_filepath(&absolute_path);

    Ok(FileProbe {
        input_name: input_name.into(),
        absolute_path,
        stat,
        mimetype,
    })
}

/// Read stdin, probe files, and read filters.
fn main() -> io::Result<()> {
    // TODO: parameterize behavior on subcommand
    let args = Args::parse();

    let _rules: Vec<Rule> = vec![Rule {
        filters: vec![
            Filter::Mimetype("image/png"),
            Filter::Mimetype("image/jpeg"),
        ],
        action: FilterAction::Move { dest: "images" },
    }];

    // reading lines of stdin, probe files
    let stdin = io::stdin();
    let files = stdin
        .lines()
        .map(|line| probe_file(&line.expect("error reading input")))
        .collect::<Vec<_>>();

    dbg!(files);

    Ok(())
}

// motivation
// - clean up files using declarative actions and filters
// - (similar to https://github.com/tfeldmann/organize)
//
// important differences
// - use dhall for configuration, not yaml
// - written in rust, not python
//
// usage
// - `cd ~/Downloads && magic-broom plan > downloads.broom`
// - `magic-broom sweep ~/Downloads.broom`
//
// design decisions
// - running `plan` cannot modify files
// - running `sweep` will not modify files that have changed since the `plan`
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
