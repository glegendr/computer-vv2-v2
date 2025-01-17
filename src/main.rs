use std::collections::HashMap;

use calculation::calc;
use commands::command_handler;
use highlighter::MatchingBracketHighlighter;
use hinter::{ComputorHinter, diy_hints};
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
mod hinter;
mod highlighter;

fn main() -> rustyline::Result<()> {
    // `()` can be used when no completer is required
    let mut variables: HashMap<String, (Option<String>, Vec<Operator>)> = HashMap::new();
    let mut chart = false;
    let mut tree = false;
    let mut quadratic_equation = true;
    let mut rl = Editor::<ComputorHinter>::new()?;
    let h = ComputorHinter {
        hints: diy_hints(),
        highlighter: MatchingBracketHighlighter::new(),
        colored_prompt: "".to_owned(),
    };
    rl.set_helper(Some(h));
    _ = rl.load_history("history.txt");
    loop {
        let p= "> ";
        rl.helper_mut().expect("No helper").colored_prompt = String::from(p);
        let readline = rl.readline(p);
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue
                }
                rl.add_history_entry(line.as_str());
                if line.starts_with("/") {
                    command_handler(&line, &mut variables, &mut rl, &mut chart, &mut tree, &mut quadratic_equation);
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
                                            if quadratic_equation {
                                                print_equation_result(&value);
                                            }
                                            if tree {
                                                if let Ok(tree) = BTree::from_vec(&first_part) {
                                                    tree.print();
                                                }
                                            }
                                            if chart {
                                                print_chart(&value);
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
                                                        if quadratic_equation {
                                                            print_equation_result(&value);
                                                        }
                                                        if tree {
                                                            if let Ok(tree) = BTree::from_vec(&first_part) {
                                                                tree.print();
                                                            }
                                                        }
                                                        if chart {
                                                            print_chart(&value);
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

fn print_equation_result(tree: &BTree) {
    if tree.all(|subtree| match subtree.node {
        Operator::Number { x, i, .. } => i == 0 && x >= 0 && x <= 2,
        Operator::Add => true,
        _ => false
    }) {
        let mut x0 = None;
        let mut x1 = None;
        let mut x2 = None;
        for ope in tree.get_all_vals() {
            match ope {
                Operator::Number { number, x, .. } => {
                    match x {
                        0 => x0 = Some(x0.unwrap_or(0.) + number),
                        1 => x1 = Some(x1.unwrap_or(0.) + number),
                        2 => x2 = Some(x2.unwrap_or(0.) + number),
                        _ => return
                    }
                },
                _ => return
            }
        }
        if x2.is_none() {
            return
        }
        let disc = x1.unwrap_or(0.).powf(2.) - x2.unwrap_or(0.) * x0.unwrap_or(0.) * 4.;
        println!("∆ = {disc}");
        if disc < 0. {
            println!("No solution on R");
        } else if disc == 0. {
            println!("A unique solution as been founded:");
            println!("x = {}", -x1.unwrap_or(0.) / (2. * x2.unwrap_or(1.)));
        } else {
            println!("Two solutions as been founded:");
            let res_x1 = (-x1.unwrap_or(0.) - disc.sqrt()) / (2. * x2.unwrap_or(1.));
            let res_x2 = (-x1.unwrap_or(0.) + disc.sqrt()) / (2. * x2.unwrap_or(1.));
            println!("x1 = {res_x1} and x2 = {res_x2}");
        }
    }
}

fn print_chart(tree: &BTree) {
    Chart::new(120, 40, -10., 10.)
    .lineplot(&Shape::Continuous(Box::new(|nb| {
        match tree.change_x(nb as f64).eval() {
            Ok(res) => {
                match res.node {
                    Operator::Number { number, .. } => {
                        if number == f64::INFINITY {
                            return -f32::INFINITY
                        }
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