use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub name: String,
    pub sources: Vec<Source>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub name: String,
    pub docs: Docs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Docs {
    pub source: String,
    pub local: String,
}

pub fn get_info(filename: &str) -> Info {
    let content =
        fs::read_to_string(filename).expect(format!("No info file at {}", &filename).as_str());
    serde_json::from_str::<Info>(&content).expect("Could not parse info file")
}
