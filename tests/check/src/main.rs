use clap::{Parser, Subcommand};

use html_parser::Dom;
use info::Info;
use struct_api::StructApi;

mod info;
mod source;
mod struct_api;

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
    /// Check if crate implements the std library API
    Implements { name: String },
}

fn get_doc_source<'a>(info: &'a Info, source: &str, name: &str) -> &'a str {
    match source {
        "std" => info
            .sources
            .iter()
            .find(|x| x.name == name)
            .unwrap()
            .docs
            .std
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

fn get_api(info: &Info, name: &str, source: &str) -> StructApi {
    let doc_source = get_doc_source(&info, &source, &name);
    let source_html = source::get(&info.name, &source, doc_source);
    let source_json = Dom::parse(&source_html).unwrap().to_json().unwrap();
    let json_value = json::parse(&source_json).unwrap();
    struct_api::StructApi::from_json(&json_value)
}

fn implements_implementations(
    local_implementations: &Vec<(String, Vec<String>)>,
    std_implementations: &Vec<(String, Vec<String>)>,
) -> bool {
    let mut success = true;
    for (name, methods) in std_implementations {
        match local_implementations.iter().find(|&x| &x.0 == name) {
            Some((_, local_methods)) => {
                for method in methods {
                    match local_methods.iter().find(|&x| x == method) {
                        Some(_) => {}
                        None => {
                            success = false;
                            println!("API does not implement: {} - {}", name, method);
                        }
                    }
                }
            }
            None => {
                success = false;
                println!("API does not implement: {}", name);
            }
        }
    }
    success
}

fn implements(local_api: &StructApi, std_api: &StructApi) -> bool {
    let mut success = true;
    let local_name: Vec<&str> = local_api.name.split("::").into_iter().skip(1).collect();
    let std_name: Vec<&str> = std_api.name.split("::").into_iter().skip(1).collect();
    if local_name != std_name {
        success = false;
        println!("API namespace do not match:");
        println!("  - {:?}", local_name);
        println!("  - {:?}", std_name);
    }

    success =
        success && implements_implementations(&local_api.implementations, &std_api.implementations);

    success = success
        && implements_implementations(
            &local_api.trait_implementations,
            &std_api.trait_implementations,
        );

    success
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
            let api = get_api(&info, &name, &source);
            println!("{}", api);
        }
        Commands::Implements { name } => {
            let info = info::get_info("./info.json");
            let local_api = get_api(&info, &name, "local");
            let std_api = get_api(&info, &name, "std");
            if implements(&local_api, &std_api) == false {
                std::process::exit(1);
            }
        }
    }
}
