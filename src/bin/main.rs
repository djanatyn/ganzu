use clap::Parser;
use ganzu::error::{Error, Ganzu};
use ganzu::example;
use ganzu::file::FileSnapshot;
use std::collections::HashMap;
use std::io;

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

/// Read stdin, probe files.
fn main() -> Ganzu<()> {
    // TODO: use arguments
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
