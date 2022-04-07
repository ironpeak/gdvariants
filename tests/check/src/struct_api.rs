use std::{collections::HashMap, fmt::Display, panic};

use json::JsonValue;

pub struct StructApi {
    name: String,
    declaration: String,
    implementations: Vec<HashMap<String, Vec<String>>>,
    trait_implementations: Vec<HashMap<String, Vec<String>>>,
}

fn fmt_implementations(
    implementations: &Vec<HashMap<String, Vec<String>>>,
    f: &mut std::fmt::Formatter<'_>,
    ident: usize,
) -> std::fmt::Result {
    let indent = format!("{:ident$}", "", ident = ident);
    write!(f, "{{")?;
    let mut impl_delim = "";
    for implementation in implementations {
        for (key, methods) in implementation {
            write!(
                f,
                "{}\n{indent}  \"{}\": [",
                impl_delim,
                key,
                indent = indent,
            )?;
            let mut method_delim = "";
            for method in methods {
                write!(
                    f,
                    "{}\n{indent}    \"{}\"",
                    method_delim,
                    method,
                    indent = indent,
                )?;
                method_delim = ",";
            }
            write!(f, "\n    ]")?;
            impl_delim = ",";
        }
    }
    write!(f, "\n{indent}}}", indent = indent)
}

impl Display for StructApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "  \"name\": \"{}\",", self.name)?;
        writeln!(f, "  \"declaration\": \"{}\",", self.declaration)?;

        write!(f, "  \"implementations\": ")?;
        fmt_implementations(&self.implementations, f, 2)?;

        write!(f, ",\n  \"trait_implementations\": ")?;
        fmt_implementations(&self.trait_implementations, f, 2)?;

        write!(f, "\n}}")
    }
}

fn prettify(str: &str) -> String {
    let mut previous = str.to_string();
    loop {
        let current = previous
            .replace("  ", " ")
            .replace("&nbsp;", " ")
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace(" ::", "::")
            .replace(":: ", "::")
            .replace(" (", "(")
            .replace("( ", "(")
            .replace(" )", ")")
            .replace(" <", "<")
            .replace(" >", ">")
            .replace("& ", "&")
            .replace("? ", "?")
            .replace(" ,", ",")
            .replace("\n", "")
            .trim()
            .to_string();
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
    panic!("Could not find element where {} = {}", key, value);
}

fn get_with_class<'a>(json: &'a JsonValue, class: &str) -> &'a JsonValue {
    for member in json.members() {
        if member["classes"].contains(class) {
            return member;
        }
    }
    panic!("Could not find element with class {}", class);
}

fn find_with_class<'a>(json: &'a JsonValue, class: &str) -> Option<&'a JsonValue> {
    if json.is_array() {
        for member in json.members() {
            match find_with_class(member, class) {
                Some(result) => return Some(result),
                None => continue,
            }
        }
    } else {
        if json["classes"].contains(class) {
            return Some(json);
        }
        if json.has_key("children") {
            return find_with_class(&json["children"], class);
        }
    }

    None
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
    panic!("Could not find element with id {}", id);
}

fn get_text<'a>(json: &'a JsonValue, ignore: &str) -> String {
    if json["classes"].contains(ignore) {
        return String::new();
    }
    let text = if json.is_array() {
        let mut text = "".to_string();
        for member in json.members() {
            if member.is_string() {
                text.push_str(format!(" {} ", member.as_str().unwrap()).as_str());
            } else {
                text.push_str(format!(" {} ", &get_text(&member, ignore)).as_str());
            }
        }
        text
    } else {
        if json.has_key("children") {
            get_text(&json["children"], ignore)
        } else {
            String::new()
        }
    };
    text
}

fn get_name(main_content: &JsonValue) -> String {
    let text = get_text(
        get_with_class(
            &find_with_class(&main_content["children"], "fqn").unwrap()["children"],
            "in-band",
        ),
        "notable-traits",
    );
    text.replace(" ", "")[6..].to_string()
}

fn get_declaration(main_content: &JsonValue) -> String {
    let text = get_text(
        get_with_class(
            &get_with_class(&main_content["children"], "item-decl")["children"],
            "struct",
        ),
        "notable-traits",
    );
    prettify(&text)
}

fn get_implementation_header(implementation: &JsonValue) -> String {
    let text = get_text(
        get_with_class(
            &get_with_class(
                &get(&implementation["children"], "name", "summary")["children"],
                "impl",
            )["children"],
            "code-header",
        ),
        "notable-traits",
    );

    prettify(&text)
}

fn get_implementation_methods(implementation: &JsonValue) -> Vec<String> {
    let mut methods = Vec::new();
    for method_container in
        get_with_class(&implementation["children"], "impl-items")["children"].members()
    {
        let method_element = &find_with_class(&method_container, "code-header").unwrap();
        let method = get_text(&method_element, "notable-traits");
        methods.push(prettify(&method));
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

fn get_trait_implementations(main_content: &JsonValue) -> Vec<HashMap<String, Vec<String>>> {
    let mut implementations = Vec::new();

    for implementation in get_all_with_class(
        &get_with_id(&main_content["children"], "trait-implementations-list")["children"],
        "implementors-toggle",
    ) {
        let trait_implementation_header = get_implementation_header(implementation);
        let mut result = HashMap::new();

        let methods = get_implementation_methods(implementation);

        result.insert(trait_implementation_header, methods);
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
        let trait_implementations = get_trait_implementations(main_content);

        return StructApi {
            name: name,
            declaration: declaration,
            implementations: implementations,
            trait_implementations: trait_implementations,
        };
    }
}
