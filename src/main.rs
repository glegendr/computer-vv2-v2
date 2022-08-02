use rustyline::Editor;
mod assignation;
mod parsing;
mod operator;
mod global;
mod calculation;

#[macro_use]
extern crate lazy_static;

use crate::operator::Operator;
use crate::assignation::assign;
use crate::parsing::parse_line;  

fn main() -> rustyline::Result<()> {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new()?;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue
                }
                rl.add_history_entry(line.as_str());
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
                            Ok(0) => assign(&splitted),
                            _ => {
                                // Calculation
                            }
                        }
                        println!("{splitted:?}");
                    },
                }
            },
            Err(_) => {
                break
            }
        }
    }
    Ok(())
}