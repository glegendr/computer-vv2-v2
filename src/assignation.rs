use std::collections::HashMap;

use crate::{operator::Operator, calculation::calc};

pub fn assign(input: &Vec<Vec<Operator>>, variables: &mut HashMap<String, Vec<Operator>>) {
    let first_part = input.get(0).unwrap();
    let mut new_sec_part = Vec::new();

    for ope in input.get(1).unwrap() {
        match ope {
            Operator::Var(name) => {
                match name.as_str() {
                    "i" | "I" => new_sec_part.push(Operator::Number { number: 1., x: 0, i: 1 }),
                    _ => {
                        match variables.get(name) {
                            Some(value) => for ope in value {
                                new_sec_part.push(ope.clone());
                            },
                            None => {
                                if name == &String::from("X") || name == &String::from("x") {
                                    new_sec_part.push(Operator::Number { number: 1., x: 1, i: 0 })
                                } else {
                                    println!("Unkwown variable {name}");
                                    return
                                }
                            }
                        } 
                    }
                }
            },
            _ => new_sec_part.push(ope.clone()),
        }
    }

    match first_part.len() {
        0 => println!("Empty input"),
        1 => variable_assignation(&vec![first_part.clone(), new_sec_part], variables),
        2 => function_assignation(&vec![first_part.clone(), new_sec_part], variables),
        _ => {
            println!("Assignation is only available for variable or function. Eq:");
            println!(">> x = 50 - 8");
            println!(">> f(x) = 3 * x + 2");
        }
    }
}

fn variable_assignation(input: &Vec<Vec<Operator>>, variables: &mut HashMap<String, Vec<Operator>>) {
    let first_part = input.get(0).unwrap();

    match first_part.get(0).unwrap() {
        Operator::Var(name) => {
            if name == "i" || name == "I" {
                println!("cannot reassign i");
                return
            }
            match calc(input.get(1).unwrap()) {
                Ok(value) => {
                    // value.print();
                    // BTree::from_vec(&mut value.clone()).unwrap_or(BTree::new(Operator::Add)).print();
                    println!("{name} = {}", to_printable_string(&value.to_vec()));
                    variables.insert(String::from(name), value.to_vec());
                },
                Err(e) => println!("{e}")
            }

        },
        _ => {
            println!("Assignation is only available for variable or function. Eq:");
            println!(">> x = 50 - 8");
        }
    }
}

fn function_assignation(input: &Vec<Vec<Operator>>, _variables: &mut HashMap<String, Vec<Operator>>) {
    let first_part = input.get(0).unwrap();

    match (first_part.get(0).unwrap(), first_part.get(1).unwrap()) {
        (Operator::Var(name), Operator::Var(value)) => {
            println!("{name}({value}) = ");
        },
        _ => {
            println!("Assignation is only available for variable or function. Eq:");
            println!(">> f(x) = 3 * x + 2");
        }
    }
}

pub fn to_printable_string(value: &Vec<Operator>) -> String {
    let new_value = from_postfix(value);
    let mut ret = String::new();
    for ope in new_value {
        ret = format!("{ret}{ope} ");
    }
    ret
}

fn from_postfix(value: &Vec<Operator>) -> Vec<Operator> {
    let mut stack: Vec<Vec<Operator>> = Vec::new();
    for ope in value {
        match ope {
            Operator::Var(_) | Operator::Number { .. } | Operator::Mat(_) => stack.push(vec![ope.clone()]),
            _ => match (stack.pop(), stack.pop()) {
                (Some(mut v1), Some(mut v2)) => {
                    let mut merge = vec![Operator::OpenParenthesis];
                    merge.append(&mut v2); 
                    merge.push(ope.clone());
                    merge.append(&mut v1); 
                    merge.push(Operator::CloseParenthesis); 
                    stack.push(merge);
                }
                _ => continue,
            }
        }
    }
    stack.iter_mut().fold(Vec::new(), |mut acc, x| {
        acc.append(x);
        acc
    })
}