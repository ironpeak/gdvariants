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
    pub std: String,
    pub local: String,
    pub overwrites: Vec<Overwrite>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overwrite {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub value: Option<String>,
    #[serde(default)]
    pub methods: Vec<Method>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Method {
    pub name: String,
    pub value: Option<String>,
}

pub fn get_info(filename: &str) -> Info {
    let content =
        fs::read_to_string(filename).expect(format!("No info file at {}", &filename).as_str());
    serde_json::from_str::<Info>(&content).expect("Could not parse info file")
}
