use std::collections::HashMap;
use rustyline::Editor;
use crate::{operator::Operator, assignation::to_printable_string};

pub fn command_handler(line: &str, variables: &mut HashMap<String, (Option<String>, Vec<Operator>)>, rl: &mut Editor<()>) {
    match line.trim_end() {
        "/history" => {
            rl.history().iter().for_each(|x| println!("{x}"));
        }
        "/list" => {
            for (key, (name, value)) in variables {
                match name {
                    Some(name) => println!("{key}({name}) = {}", to_printable_string(value)),
                    None => println!("{key} = {}", to_printable_string(value))
                }
            }
        },
        _ => {}
    }
}