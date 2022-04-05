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
    ListSources {},
    /// Get API for source
    Api {},
    /// Show differences between sources
    Diff {},
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::ListSources {} => {
            let info = info::get_info("./info.json");
            for source in info.sources {
                println!("{}", source.name);
            }
        }
        Commands::Api {} => todo!(),
        Commands::Diff {} => todo!(),
    }
}
