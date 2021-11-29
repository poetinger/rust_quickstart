//! My newest Rust program needs a description.
use std::path;

/// The main commands and flags.
#[derive(Debug, structopt::StructOpt)]
#[structopt(author)]
pub (crate) struct Arguments {
    /// Subcommand to run.
    #[structopt(subcommand)]
    pub(crate) subcommand: Subcommand,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub(crate) verbose: u8,
}

/// Main subcommands of the program.
#[derive(Debug, structopt::StructOpt)]
pub (crate) enum Subcommand {
    /// Basic subcommand
    Run,
}
