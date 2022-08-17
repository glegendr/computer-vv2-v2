use std::collections::HashMap;

use assignation::change_known_variables;
use calculation::calc;
use rustyline::Editor;
mod assignation;
mod parsing;
mod operator;
mod calculation;
mod btree;

use crate::operator::Operator;
use crate::assignation::{assign, to_printable_string};
use crate::parsing::parse_line;  

fn main() -> rustyline::Result<()> {
    // `()` can be used when no completer is required
    let mut variables: HashMap<String, (Option<String>, Vec<Operator>)> = HashMap::new();
    let mut rl = Editor::<()>::new()?;
    _ = rl.load_history("history.txt");
    'main_loop: loop {
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

                match parse_line(line.as_str()) {
                    Err(e) => println!("{e}"),
                    Ok(operators) => {
                        let mut splitted: Vec<Vec<Operator>> = operators
                            .split(|ope| ope == &Operator::Equal)
                            .map(|ope| ope.to_vec())
                            .collect();
                        match splitted.len() {
                            0..=1 => {
                                println!("expect 1 equal");
                                continue
                            }
                            2 => {}
                            _ => {
                                println!("too mutch equal found");
                                continue
                            }
                        }
                        match splitted.iter().enumerate().fold(Ok(0), |acc, (i, x)| {
                            let mut nb = acc?;
                            nb += x.iter().filter(|ope| ope == &&Operator::Var(String::from("?"))).count();
                            if nb > 1 {
                                return Err(String::from("Multiple ?"))
                            } else if nb != 0 && i == 0 {
                                return Err(String::from("? in first part"))
                            }
                            Ok(nb)
                        }) {
                            Err(e) => {
                                println!("{e}");
                                continue
                            }
                            Ok(0) => assign(&splitted, &mut variables),
                            _ => {
                                // Calculation
                                match splitted.last_mut() {
                                    Some(last) => {
                                        match last.pop() {
                                            Some(last) => {
                                                match last {
                                                    Operator::Var(name) => match name.as_str() {
                                                        "?" => {},
                                                        _ => {
                                                            println!("Expected ? at the end !");
                                                            continue;
                                                        }
                                                    },
                                                    _ => continue
                                                }
                                            },
                                            _ => continue
                                        }
                                    }
                                    _ => continue
                                }
                                let mut merged = Vec::new();
                                for part in &splitted {
                                    if part.is_empty() && merged.is_empty() {
                                        println!("Empty first part");
                                        continue 'main_loop
                                    } else if part.is_empty() {
                                        continue
                                    }
                                    match change_known_variables(part, &variables, None) {
                                        Ok(mut res) => {
                                            if merged.is_empty() {
                                                merged = res;
                                            } else {
                                                merged.append(&mut res);
                                                merged.push(Operator::Minus);
                                            }
                                        },
                                        Err(e) => {
                                            println!("{e}");
                                            continue 'main_loop;
                                        }
                                    }
                                }
                                match calc(&merged) {
                                    Ok(value) => {
                                        println!("{}", to_printable_string(&value.to_vec()));
                                    },
                                    Err(e) => {
                                        println!("{e}");
                                        continue;
                                    }
                                }
                            }
                        }
                    },
                }
            },
            Err(_) => {
                break
            }
        }
    }
    rl.save_history("history.txt")
}