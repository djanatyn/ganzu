///! organize files according to rules
///!
///! testing:
///! TODO: add test directory
///! TODO: test parsing example manifest
///! TODO: test generating / parsing kdl for Meta
///! TODO: test generating / parsing kdl for FileSnapshot
///!
///! control flow:
///! TODO: fail if unable to snapshot files
///! TODO: fail if unable to match files
///!
///! output:
///! TODO: report files successfully snapshotted
///! TODO: report files matched
///! TODO: output kdl manifest
///!
use clap::Parser;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

mod args;
mod config;
mod error;
mod example;
mod file;
mod filter;
mod kdl;
mod rule;

use args::Args;
use config::Config;
use error::{Error, Ganzu};
use file::FileSnapshot;
use filter::{Filter, FilterAction};
use rule::Rule;

/// Read stdin, probe files.
fn main() -> Ganzu<()> {
    let args = Args::parse();

    // define filters and actions
    let config = example::config();

    // get input and probe files
    let mut snapshots: HashMap<String, FileSnapshot> = HashMap::new();
    let mut failed: HashMap<String, Ganzu<FileSnapshot>> = HashMap::new();
    let stdin = io::stdin();
    for line in stdin.lines() {
        let text = line.map_err(Error::InputFailure)?;
        // collect success + failure
        match FileSnapshot::new(&text) {
            Ok(probe) => {
                snapshots.insert(text.to_string(), probe);
            }
            failure => {
                failed.insert(text.to_string(), failure);
            }
        };
    }

    let total = snapshots.len() + failed.len();
    println!("probed {0}/{1} files successfully", snapshots.len(), total);

    // exit if there were any failures probing file (prompt the user to resolve)
    if failed.len() != 0 {
        // TODO: exit
    }

    // start matching against our rules
    for file in snapshots.values() {
        for rule in &config.rules {
            if let Some(filter) = rule.check(file) {
                // match
                dbg!((filter, file));
            }
        }
    }

    println!("done");

    Ok(())
}
