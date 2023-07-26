use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use regex::{Captures, Regex};
use tracing::error;

pub enum Data {
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

pub fn render(template: &str, data: HashMap<&str, Data>) -> String {
    let print_regex = Regex::new(r"\{\{(.*?)\}\}").unwrap();
    let new_template = print_regex.replace_all(&template, |caps: &Captures| {
        let key = caps.get(1).unwrap().as_str().trim();
        data[key].to_string()
    });

    let repeat_regex = Regex::new(r"\{% repeat (\d*?) %\}((.|\n)*?)\{% end %\}").unwrap();
    let new_template = repeat_regex.replace_all(&new_template, |caps: &Captures| {
        let times = caps.get(1).unwrap().as_str().trim();
        let code = caps.get(2).unwrap().as_str().trim();

        code.repeat(times.parse::<usize>().unwrap())
    });

    let if_else_regex = Regex::new(
        r"\{% if (.*?) %\}((.|\n)*?)(\{% else %\}((.|\n)*?)\{% endif %\}|\{% endif %\})",
    )
    .unwrap();

    let new_template = if_else_regex.replace_all(&new_template, |caps: &Captures| {
        let key = caps.get(1).unwrap().as_str().trim();
        let if_code = caps.get(2).unwrap().as_str().trim();
        let else_code = caps.get(5).map_or("", |m| m.as_str().trim());

        if let Data::Boolean(exp) = data[key] {
            if exp {
                if_code.to_string()
            } else {
                else_code.to_string()
            }
        } else {
            error!(
                "Could not parse boolean key as boolean, or key didn't exist {}",
                key
            );
            panic!();
        }
    });
    // {# ... #}
    // <-- ... -->
    let new_template = new_template.replace("{#", "<--").replace("#}", "-->");

    new_template.to_string()
}

#[test]
fn basic_template() {
    let input = std::fs::read_to_string("templates/index.html").unwrap();
    let data = HashMap::from([
        ("name", Data::Text("Bob".to_string())),
        ("allowed", Data::Boolean(true)),
    ]);

    let render = render(&input, data);
    println!("{}", &render);
    assert!(render.find("Welcome to hello world").is_some())
}
