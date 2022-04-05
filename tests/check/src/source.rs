use std::fs;

pub fn get(name: &str, source_type: &str, source_value: &str) -> String {
    match source_type {
        "local" => fs::read_to_string(format!("./target/doc/{}/{}", name, source_value)).unwrap(),
        &_ => todo!(),
    }
}
