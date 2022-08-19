use std::collections::HashMap;
use colored::Colorize;
use rustyline::Editor;
use crate::{operator::Operator, assignation::to_printable_string};

pub fn command_handler(line: &str, variables: &mut HashMap<String, (Option<String>, Vec<Operator>)>, rl: &mut Editor<()>, chart_enabled: &mut bool) {
    let splitted: Vec<String> = line.split(" ").map(|cmd| String::from(cmd.trim())).collect();
    if splitted.is_empty() {
        return
    }
    match splitted.get(0).unwrap().as_str() {
        "/history" => history(splitted[1..].to_vec(), rl),
        "/list" => list(splitted[1..].to_vec(), variables),
        "/clear" => clear(splitted[1..].to_vec(), variables, rl),
        "/chart" => chart(splitted[1..].to_vec(), chart_enabled),
        "/help" => help(splitted[1..].to_vec()),
        cmd => println!("unknown command {cmd} try /help")
    }
}

fn help(splitted: Vec<String>) {
    match splitted.is_empty() {
        true => help(vec![String::from("*")]),
        false => {
            for cmd in splitted {
                match cmd.to_lowercase().as_str() {
                    "*" | "all" => help(vec![String::from("cmd"), String::from("ass"), String::from("calc")]),
                    "cmd" | "commands" => {
                        println!("{}", "----------- Commands ------------".bold().purple());
                        println!("{} {} ..", "/history".purple(), "<?pattern>".yellow());
                        println!("display all history or filtered if patern is given\n");
                        println!("{} {} ..", "/list".purple(), "<?ima> <?fn> <?rat> <?mat>".yellow());
                        println!("list all variables sorted by type or selected one\n");
                        println!("{} {} ..", "/clear".purple(), "<?history> <?variables> <?*>".yellow());
                        println!("clear history and variables or only selected one\n");
                        println!("{} {} {} ..", "/chart".purple(), "<?on>".green(), "<?off>".red());
                        println!("toggle chart\n");
                        println!("{} {} ..", "/help".purple(),  "<?cmd> <?ass> <?calc> <?*>".yellow());
                        println!("display helping message about commands, assignation and calculus\n");
                    }
                    "ass" | "assignation" | "=" => {
                        println!("{}", "---------- Assignation ----------".bold().purple());
                        println!("Assign a variable or a function to use it in calculus after.");
                        println!("Rationnal:");
                        println!("{} = 32 + 10\n", "myRatVar".purple());
                        println!("Imaginary:");
                        println!("{} = (32 + 10) * i\n", "myImaVar".purple());
                        println!("Matrix:");
                        println!("{} = [[1,2,3];[4,5,6];[7,8,9]] * 2\n", "myMatrix".purple());
                        println!("Function:");
                        println!("{}({1}) = (32 + {1}) * 10\n", "myFn".purple(), "myFnVar".yellow());
                    }
                    "calc" | "calculus" | "?" => {
                        println!("{}", "----------- Calculus ------------".bold().purple());
                        println!("To do a calculus simply use:");
                        println!("-2 + 32 + 3*4 = ?\n");
                        println!("This will give you {}", "42".yellow());
                    }
                    _ => {}
                }
            }
        }
    }
}

fn chart(splitted: Vec<String>, chart_enabled: &mut bool) {
    match splitted.is_empty() {
        true => *chart_enabled = !*chart_enabled,
        false => {
            for cmd in splitted {
                match cmd.to_lowercase().as_str() {
                    "on" | "true" => *chart_enabled = true,
                    "off" | "false" => *chart_enabled = false,
                    _ => {}
                }
            }
        }
    }
    match *chart_enabled {
        true => println!("{}", "+ chart".green()),
        false => println!("{}", "- chart".red())
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