use clap::{Parser, Subcommand};

use info::Info;

mod info;
mod source;

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
    GetApi { name: String, source: String },
}

fn get_doc_source<'a>(info: &'a Info, source: &str, name: &str) -> &'a str {
    match source {
        "source" => info
            .sources
            .iter()
            .find(|x| x.name == name)
            .unwrap()
            .docs
            .source
            .as_str(),
        "local" => info
            .sources
            .iter()
            .find(|x| x.name == name)
            .unwrap()
            .docs
            .local
            .as_str(),
    }
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
        Commands::GetApi { name, source } => {
            let info = info::get_info("./info.json");
            let doc_source = get_doc_source(&info, &source, &name);
            let source = source::get(&info.name, &source, doc_source);
        }
    }
}
