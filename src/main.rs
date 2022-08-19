use std::collections::HashMap;

use calculation::calc;
use commands::command_handler;
use parsing::ActionType;
use rustyline::Editor;
mod assignation;
mod parsing;
mod operator;
mod calculation;
mod btree;
mod commands;

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
                if line.starts_with("/") {
                    command_handler(&line, &mut variables, &mut rl);
                    continue
                }
                match parse_line(line.as_str(), &variables) {
                    Err(e) => println!("{e}"),
                    Ok(ActionType::Calculus(input)) => {
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