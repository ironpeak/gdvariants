use std::collections::HashMap;

use json::JsonValue;

#[derive(Debug)]
pub struct StructApi {
    name: String,
    declaration: String,
    implementations: Vec<HashMap<String, Vec<String>>>,
}

fn get<'a>(json: &'a JsonValue, key: &str, value: &str) -> &'a JsonValue {
    for member in json.members() {
        if member[key] == value {
            return member;
        }
    }
    panic!();
}

fn get_with_class<'a>(json: &'a JsonValue, class: &str) -> &'a JsonValue {
    for member in json.members() {
        if member["classes"].contains(class) {
            return member;
        }
    }
    panic!();
}

fn get_with_id<'a>(json: &'a JsonValue, id: &str) -> &'a JsonValue {
    for member in json.members() {
        if member["id"] == id {
            return member;
        }
    }
    panic!();
}

fn get_text<'a>(json: &'a JsonValue) -> String {
    let text = if json.is_array() {
        let mut text = "".to_string();
        for member in json.members() {
            if member.is_string() {
                text.push_str(member.as_str().unwrap());
            } else {
                text.push_str(&get_text(&member));
            }
        }
        text
    } else {
        if json.has_key("children") {
            get_text(&json["children"])
        } else {
            String::new()
        }
    };
    text.replace("&nbsp;", " ")
        .replace("&quot;", "\"")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&#39;", "'")
}

fn get_name(main_content: &JsonValue) -> String {
    let text = get_text(get_with_class(
        &get_with_class(&main_content["children"], "fqn")["children"],
        "in-band",
    ));
    text[6..].to_string()
}

fn get_declaration(main_content: &JsonValue) -> String {
    let text = get_text(get_with_class(
        &get_with_class(&main_content["children"], "item-decl")["children"],
        "struct",
    ));
    text.to_string()
}

fn get_implementations(main_content: &JsonValue) -> Vec<HashMap<String, Vec<String>>> {
    let implementations = Vec::new();

    implementations
}

impl StructApi {
    pub fn from_json(json: &JsonValue) -> StructApi {
        let html_element = get(&json["children"], "name", "html");
        let body_element = get(&html_element["children"], "name", "body");
        let main_element = get(&body_element["children"], "name", "main");
        let div_width_limiter = get_with_class(&main_element["children"], "width-limiter");
        let main_content = get_with_id(&div_width_limiter["children"], "main-content");

        let name = get_name(main_content);
        let declaration = get_declaration(main_content);
        let implementations = get_implementations(main_content);

        return StructApi {
            name: name,
            declaration: declaration,
            implementations: implementations,
        };
    }
}
