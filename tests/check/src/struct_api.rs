use std::{collections::HashMap, fmt::Display, mem::replace};

use json::JsonValue;

pub struct StructApi {
    name: String,
    declaration: String,
    implementations: Vec<HashMap<String, Vec<String>>>,
}

impl Display for StructApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "name: {}", self.name)?;
        writeln!(f, "declaration: {}", self.declaration)?;
        writeln!(f, "implementations: [")?;
        for implementation in &self.implementations {
            for (implementation, methods) in implementation {
                writeln!(f, "  {}: [", implementation)?;
                for method in methods {
                    writeln!(f, "    {}", method)?;
                }
                writeln!(f, "  ]")?;
            }
        }
        writeln!(f, "]")
    }
}

fn remove_multiple_spaces(str: &str) -> String {
    let mut previous = str.to_string();
    loop {
        let current = previous.replace("  ", " ");
        if current == previous {
            return current;
        }
        previous = current;
    }
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

fn get_all_with_class<'a>(json: &'a JsonValue, class: &str) -> Vec<&'a JsonValue> {
    let mut values = Vec::new();
    for member in json.members() {
        if member["classes"].contains(class) {
            values.push(member);
        }
    }
    values
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

fn get_implementation_header(implementation: &JsonValue) -> String {
    let text = get_text(get_with_class(
        &get_with_class(
            &get(&implementation["children"], "name", "summary")["children"],
            "impl",
        )["children"],
        "code-header",
    ));
    text.to_string()
}

fn get_implementation_methods(implementation: &JsonValue) -> Vec<String> {
    let mut methods = Vec::new();
    for method in get_all_with_class(
        &get_with_class(&implementation["children"], "impl-items")["children"],
        "method-toggle",
    ) {
        let method = get_text(&get_with_class(
            &get_with_class(
                &get(&method["children"], "name", "summary")["children"],
                "method",
            )["children"],
            "code-header",
        ))
        .replace("pub fn", "pub fn ")
        .replace(">where", "> where")
        .replace(")where", ") where")
        .replace("->", "-> ")
        .replace("+", " + ")
        .replace(":", ": ")
        .replace("\n", "");
        methods.push(remove_multiple_spaces(&method));
    }
    methods
}

fn get_implementations(main_content: &JsonValue) -> Vec<HashMap<String, Vec<String>>> {
    let mut implementations = Vec::new();

    for implementation in get_all_with_class(&main_content["children"], "implementors-toggle") {
        let implementation_header = get_implementation_header(implementation);
        let mut result = HashMap::new();

        let methods = get_implementation_methods(implementation);

        result.insert(implementation_header, methods);
        implementations.push(result);
    }

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
