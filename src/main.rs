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

use crate::btree::BTree;
use crate::operator::Operator;
use crate::assignation::{to_printable_string};
use crate::parsing::parse_line;  
use textplots::{Chart, Plot, Shape};

fn main() -> rustyline::Result<()> {
    // `()` can be used when no completer is required
    let mut variables: HashMap<String, (Option<String>, Vec<Operator>)> = HashMap::new();
    let mut chart = false;
    let mut tree = false;
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
                    command_handler(&line, &mut variables, &mut rl, &mut chart, &mut tree);
                    continue
                }
                match parse_line(line.as_str(), &variables) {
                    Err(e) => println!("{e}"),
                    Ok(ActionType::Calculus((mut first_part, mut sec_part))) => {
                        if sec_part.is_empty() {
                            match calc(&first_part) {
                                Ok(value) => {
                                    match value.to_vec() == vec![Operator::Number { number: 0., x: 0, i: 0 }] {
                                        true => println!("true"),
                                        false => {
                                            println!("{}", to_printable_string(&value.to_vec()));
                                            if tree {
                                                if let Ok(tree) = BTree::from_vec(&first_part) {
                                                    tree.print();
                                                }
                                            }
                                            if chart {
                                                print_chart(&first_part);
                                            }
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("{e}");
                                    continue;
                                }
                            }
                        } else {
                            match (calc(&first_part), calc(&sec_part)) {
                                (Err(e), _) | (_, Err(e)) => println!("{e}"),
                                (Ok(v1), Ok(v2)) => match v1 == v2 {
                                    true => println!("true"),
                                    false => {
                                        first_part.append(&mut sec_part);
                                        first_part.append(&mut vec![Operator::Number { number: -1., x: 0, i: 0 }, Operator::Mult, Operator::Add]);
                                        match calc(&first_part) {
                                            Ok(value) => {
                                                match value.to_vec() == vec![Operator::Number { number: 0., x: 0, i: 0 }] {
                                                    true => println!("true"),
                                                    false => {
                                                        println!("{}", to_printable_string(&value.to_vec()));
                                                        if tree {
                                                            if let Ok(tree) = BTree::from_vec(&first_part) {
                                                                tree.print();
                                                            }
                                                        }
                                                        if chart {
                                                            print_chart(&first_part);
                                                        }
                                                    }
                                                }
                                            },
                                            Err(e) => {
                                                println!("{e}");
                                                continue;
                                            }
                                        }
                                    }
                                }
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

fn print_chart(input: &Vec<Operator>) {
    Chart::new(120, 40, -10., 10.)
    .lineplot(&Shape::Continuous(Box::new(|nb| {
        let changed = input.iter().map(|ope| {
            match ope {
                Operator::Number { number, x, i } => Operator::Number { number: number * (nb as f64).powf(*x as f64), x: 0, i: *i },
                _ => ope.clone()
            }
        })
        .collect();
        match calc(&changed) {
            Ok(res) => {
                match res.node {
                    Operator::Number { number, .. } => {
                        if number >= f32::MAX as f64 {
                            f32::MAX - 1.
                        } else {
                            number as f32
                        }
                    },
                    _ => 0.,
                }
            },
            _ => 0.
        }
    })))
    .display();
}