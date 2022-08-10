use std::collections::HashMap;

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
    let mut variables = HashMap::new();
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
                    "/list" => {
                        for (key, value) in &variables {
                            println!("{key} = {}", to_printable_string(value));
                        }
                        continue
                    },
                    _ => {}
                }

                match parse_line(line.as_str()) {
                    Err(e) => println!("{e}"),
                    Ok(operators) => {
                        let splitted: Vec<Vec<Operator>> = operators
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
                            }
                        }
                        // println!("{splitted:?}");
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