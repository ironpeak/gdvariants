use clap::{Parser, Subcommand};

mod info;

#[derive(Debug, Parser)]
#[clap(name = "check")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List sources
    #[clap(arg_required_else_help = true)]
    ListSources {},
    /// Get API for source
    #[clap(arg_required_else_help = true)]
    Api {},
    /// Show differences between sources
    #[clap(arg_required_else_help = true)]
    Diff {},
}

fn main() {
    let _args = Cli::parse();
}
