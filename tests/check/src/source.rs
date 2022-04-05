use std::panic;

pub fn get(name: &str, source_type: &str, source_value: &str) -> String {
    match source_type {
        "local" => "".to_string(),
        &_ => todo!(),
    }
}
