use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use regex::Regex;

use crate::error::AppError;

enum Data {
    Text(String),
    Boolean(bool),
    Number(i32),
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Data::Text(x) => write!(f, "{}", x),
            Data::Boolean(x) => write!(f, "{}", x),
            Data::Number(x) => write!(f, "{}", x),
        }
    }
}

pub fn render(template: &str, data: HashMap<str, Data>) -> Result<String, AppError> {
    let replace_regex = Regex::new(r"\{\{(.*?)\}\}").unwrap();
    let validate_regex = Regex::new(r"[ 	]*([a-z][a-z0-9_]*?)[ 	]*").unwrap();

    let mut processed_template = template.to_string();
    let replaced = replace_regex.captures_iter(&processed_template);

    for cap in replaced {
        let group_to_process = cap.get_mut(1).unwrap();
        let valid = validate_regex.find(group_to_process);
        match valid {
            None => Err(AppError::Template(format!(
                "The template has an improper substitution at {}",
                group_to_process
            ))),
            Some(thing) => {}
        };
    }

    todo!()
}

// Hello {{ name }} String
