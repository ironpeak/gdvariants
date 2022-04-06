use json::JsonValue;

pub struct StructApi {
    name: String,
}

fn get<'a>(json: &'a JsonValue, key: &str, value: &str) -> &'a JsonValue {
    for member in json.members() {
        if member[key] == value {
            return member;
        }
    }
    todo!();
}

fn get_with_class<'a>(json: &'a JsonValue, class: &str) -> &'a JsonValue {
    for member in json.members() {
        if member["classes"].contains(class) {
            return member;
        }
    }
    todo!();
}

fn get_with_id<'a>(json: &'a JsonValue, id: &str) -> &'a JsonValue {
    for member in json.members() {
        if member["id"] == id {
            return member;
        }
    }
    todo!();
}

fn get_text<'a>(json: &'a JsonValue) -> String {
    if json.is_array() {
        let mut text = "".to_string();
        for member in json.members() {
            if member.is_string() {
                text.push_str(member.as_str().unwrap());
            } else {
                text.push_str(&get_text(&member));
            }
        }
        return text;
    } else {
        if json.has_key("children") {
            return get_text(&json["children"]);
        } else {
            return String::new();
        }
    }
}

fn get_name(main_content: &JsonValue) -> String {
    let text = get_text(get_with_class(
        &get_with_class(&main_content["children"], "fqn")["children"],
        "in-band",
    ));
    text[6..].to_string()
}

impl StructApi {
    pub fn from_json(json: &JsonValue) -> StructApi {
        let html_element = get(&json["children"], "name", "html");
        let body_element = get(&html_element["children"], "name", "body");
        let main_element = get(&body_element["children"], "name", "main");
        let div_width_limiter = get_with_class(&main_element["children"], "width-limiter");
        let main_content = get_with_id(&div_width_limiter["children"], "main-content");

        let name = get_name(main_content);

        return StructApi { name: name };
    }
}
