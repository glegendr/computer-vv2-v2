use std::collections::HashMap;
use colored::Colorize;
use rustyline::Editor;
use crate::{operator::Operator, assignation::to_printable_string};

pub fn command_handler(line: &str, variables: &mut HashMap<String, (Option<String>, Vec<Operator>)>, rl: &mut Editor<()>) {
    let splitted: Vec<String> = line.split(" ").map(|cmd| String::from(cmd.trim())).collect();
    if splitted.is_empty() {
        return
    }
    match splitted.get(0).unwrap().as_str() {
        "/history" => history(splitted[1..].to_vec(), rl),
        "/list" => list(splitted[1..].to_vec(), variables),
        "/clear" => clear(splitted[1..].to_vec(), variables, rl),
        cmd => println!("unknown command {cmd}")
    }
}

fn clear(splitted: Vec<String>, variables: &mut HashMap<String, (Option<String>, Vec<Operator>)>, rl: &mut Editor<()>) {
    match splitted.is_empty() {
        true => {
            variables.clear();
            rl.clear_history();
        }
        false => {
            for cmd in splitted {
                match cmd.to_lowercase().as_str() {
                    "*" | "all" => {
                        variables.clear();
                        rl.clear_history();
                    }
                    "history" | "hist" => rl.clear_history(),
                    "var" | "variables" => variables.clear(),
                    _ => {
                        if let Some(rem) = variables.remove(&cmd) {
                            let res = match rem.0 {
                                Some(v_name) => format!("- {cmd}({v_name}) = {}", to_printable_string(&rem.1)),
                                None => format!("- {cmd} = {}", to_printable_string(&rem.1))
                            };
                            println!("{}", res.red())
                        }
                    }
                }
            }
        }
    }
}

fn history(splitted: Vec<String>, rl: &mut Editor<()>) {
    match splitted.is_empty() {
        true => rl.history().iter().for_each(|x| println!("{x}")),
        false => {
            for cmd in splitted {
                println!("---------- {cmd} ----------");
                rl.history().iter().for_each(|x| if x.contains(&cmd) && !x.starts_with("/") {
                    println!("{x}")
                })
            }
        }
    }
}

fn list(splitted: Vec<String>, variables: &HashMap<String, (Option<String>, Vec<Operator>)>) {
    match splitted.is_empty() {
        true => list(vec![String::from("i"), String::from("r"), String::from("f"), String::from("m")], variables),
        false => {
            for cmd in splitted {
                match cmd.to_lowercase().as_str() {
                    "ima" | "i" | "imaginary" => list_ima(&variables),
                    "fn" | "f" | "fun" | "functions" => list_fn(variables),
                    "rat" | "r" | "rationnals" => list_rat(variables),
                    "mat" | "m" | "matrix" => list_mat(variables),
                    _ => {}
                }
            }
        }

    }
}

fn list_ima(variables: &HashMap<String, (Option<String>, Vec<Operator>)>) {
    let mut ret = String::new();
    for (key, (name, value)) in variables {
        if name.is_none() && value.iter().any(|ope| match ope {
            &Operator::Number { i, .. } => i != 0,
            _ => false
        }) {
            ret = format!("{ret}{key} = {}\n", to_printable_string(value));
        }
    }
    if !ret.is_empty() {
        println!("---------- Imaginary -----------\n{ret}");
    }
}

fn list_rat(variables: &HashMap<String, (Option<String>, Vec<Operator>)>) {
    let mut ret = String::new();
    for (key, (name, value)) in variables {
        if name.is_none() && value.iter().all(|ope| match ope {
            &Operator::Number { i, .. } => i == 0,
            &Operator::Mat(_) => false,
            _ => true
        }) {
            ret = format!("{ret}{key} = {}\n", to_printable_string(value));
        }
    }
    if !ret.is_empty() {
        println!("---------- Rationnals ----------\n{ret}");
    }
}

fn list_fn(variables: &HashMap<String, (Option<String>, Vec<Operator>)>) {
    let mut ret = String::new();
    for (key, (name, value)) in variables {
        if let Some(v_name) = name {
            ret = format!("{ret}{key}({v_name}) = {}\n", to_printable_string(value));
        }
    }
    if !ret.is_empty() {
        println!("---------- Functions -----------\n{ret}");
    }
}

fn list_mat(variables: &HashMap<String, (Option<String>, Vec<Operator>)>) {
    let mut ret = String::new();
    for (key, (name, value)) in variables {
        if name.is_none() && value.iter().any(|ope| match ope {
            &Operator::Mat(_) => true,
            _ => false
        }) {
            ret = format!("{ret}{key} = {}\n", to_printable_string(value));
        }
    }
    if !ret.is_empty() {
        println!("------------ Matrix ------------\n{ret}");
    }
}