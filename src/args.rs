use clap::Parser;

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
