// use std::{collections::HashMap, slice::Iter};

use crate::{operator::Operator, calculation::calc};

// pub fn assign(input: &Vec<Vec<Operator>>, variables: &mut HashMap<String, (Option<String>, Vec<Operator>)>) {
//     let first_part = input.get(0).unwrap();
//     let new_sec_part = match change_known_variables(input.get(1).unwrap() , variables, first_part.get(1)) {
//         Ok(res) => res,
//         Err(e) => {
//             println!("{e}");
//             return
//         }
//     };

//     match first_part.len() {
//         0 => println!("Empty input"),
//         1 => variable_assignation(&vec![first_part.clone(), new_sec_part], variables),
//         2 => function_assignation(&vec![first_part.clone(), new_sec_part], variables),
//         _ => {
//             println!("Assignation is only available for variable or function. Eq:");
//             println!(">> x = 50 - 8");
//             println!(">> f(x) = 3 * x + 2");
//         }
//     }
// }

// fn variable_assignation(input: &Vec<Vec<Operator>>, variables: &mut HashMap<String, (Option<String>, Vec<Operator>)>) {
//     let first_part = input.get(0).unwrap();

//     match first_part.get(0).unwrap() {
//         Operator::Var(name) => {
//             if name == "i" || name == "I" {
//                 println!("Cannot reassign i");
//                 return
//             }
//             match calc(input.get(1).unwrap()) {
//                 Ok(value) => {
//                     println!("{name} = {}", to_printable_string(&value.to_vec()));
//                     variables.insert(String::from(name), (None, value.to_vec()));
//                 },
//                 Err(e) => println!("{e}")
//             }
//         },
//         _ => {
//             println!("Assignation is only available for variable or function. Eq:");
//             println!(">> x = 50 - 8");
//         }
//     }
// }

// fn function_assignation(input: &Vec<Vec<Operator>>, variables: &mut HashMap<String, (Option<String>, Vec<Operator>)>) {
//     let first_part = input.get(0).unwrap();

//     match (first_part.get(0).unwrap(), first_part.get(1).unwrap()) {
//         (Operator::Var(name), Operator::Var(v_name)) => {
//             if name == "i" || name == "I" {
//                 println!("Cannot reassign i");
//                 return
//             }
//             match calc(input.get(1).unwrap()) {
//                 Ok(value) => {
//                     println!("{name}({v_name}) = {}", to_printable_string(&value.to_vec()));
//                     variables.insert(String::from(name), (Some(String::from(v_name)), value.to_vec()));
//                 },
//                 Err(e) => println!("{e}")
//             }
//         },
//         _ => {
//             println!("Assignation is only available for variable or function. Eq:");
//             println!(">> f(x) = 3 * x + 2");
//         }
//     }
// }

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
    let mut ret = stack.iter_mut().fold(Vec::new(), |mut acc, x| {
        acc.append(x);
        acc
    });
    if ret.len() > 2 {
        ret = ret[1..ret.len() - 1].to_vec();
    }
    ret
}