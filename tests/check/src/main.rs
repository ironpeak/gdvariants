use clap::{Parser, Subcommand};

use html_parser::Dom;
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
        &_ => todo!(),
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
            let source_xml = source::get(&info.name, &source, doc_source);
            let source_json = Dom::parse(&source_xml).unwrap().to_json().unwrap();
            let docs = json::parse(&source_json).unwrap();
            println!("{}", docs["children"].pretty(4));
        }
    }
}
