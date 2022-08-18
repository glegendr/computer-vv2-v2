use std::collections::HashMap;

use calculation::calc;
use parsing::ActionType;
use rustyline::Editor;
mod assignation;
mod parsing;
mod operator;
mod calculation;
mod btree;

use crate::operator::Operator;
use crate::assignation::{to_printable_string};
use crate::parsing::parse_line;  

fn main() -> rustyline::Result<()> {
    // `()` can be used when no completer is required
    let mut variables: HashMap<String, (Option<String>, Vec<Operator>)> = HashMap::new();
    let mut rl = Editor::<()>::new()?;
    _ = rl.load_history("history.txt");
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue
                }
                rl.add_history_entry(line.as_str());
                match line.trim_end() {
                    "/history" => {
                        rl.history().iter().for_each(|x| println!("{x}"));
                        continue
                    }
                    "/list" => {
                        for (key, (name, value)) in &variables {
                            match name {
                                Some(name) => println!("{key}({name}) = {}", to_printable_string(value)),
                                None => println!("{key} = {}", to_printable_string(value))
                            }
                        }
                        continue
                    },
                    _ => {}
                }

                match parse_line(line.as_str(), &variables) {
                    Err(e) => println!("{e}"),
                    Ok(ActionType::Calculus(input)) => {
                        println!("{input:?}");
                        match calc(&input) {
                            Ok(value) => {
                                println!("{}", to_printable_string(&value.to_vec()));
                            },
                            Err(e) => {
                                println!("{e}");
                                continue;
                            }
                        }
                    }
                    Ok(ActionType::VarAssignation((var_name, input))) => {
                        println!("{input:?}");
                        match calc(&input) {
                            Ok(value) => {
                                println!("{var_name} = {}", to_printable_string(&value.to_vec()));
                                variables.insert(var_name, (None, value.to_vec()));
                            },
                            Err(e) => {
                                println!("{e}");
                                continue;
                            }
                        }
                    },
                    Ok(ActionType::FunAssignation((fn_name, var_name, input))) => {
                        println!("{input:?}");
                        match calc(&input) {
                            Ok(value) => {
                                println!("{fn_name}({var_name}) = {}", to_printable_string(&value.to_vec()));
                                variables.insert(fn_name, (Some(var_name), value.to_vec()));
                            },
                            Err(e) => {
                                println!("{e}");
                                continue;
                            }
                        } 
                    }
                }
            },
            Err(_) => {
                break
            }
        }
    }
    rl.save_history("history.txt")
}