use std::{collections::HashMap, slice::Iter};

use crate::{operator::Operator, assignation::from_postfix};

pub enum ActionType {
    Calculus((Vec<Operator>, Vec<Operator>)),
    VarAssignation((String, Vec<Operator>)),
    FunAssignation((String, String, Vec<Operator>))
}

fn get_function_value(input: &mut Iter<Operator>, variables: &HashMap<String, (Option<String>, Vec<Operator>)>, value: &(Option<String>, Vec<Operator>)) -> Result<Vec<Operator>, String> {
    if value.0.is_none() {
        return Ok(from_postfix(&value.1))
    }
    let mut by = Vec::new();
    let mut depth = 0;

    while let Some(ope) = input.next() {
        match ope {
            Operator::OpenParenthesis => {
                by.push(Operator::OpenParenthesis);
                depth += 1
            },
            Operator::CloseParenthesis => {
                by.push(Operator::CloseParenthesis);
                match depth {
                    d if d <= 0 => Err("Missmatched Parentesis")?,
                    1 => break,
                    _ => depth -= 1,
                }
            },
            Operator::Var(name) => {
                match name.as_str() {
                    "i" | "I" => by.push(Operator::Number { number: 1., x: 0, i: 1 }),
                    _ => {
                        match variables.get(name) {
                            Some(value) => by.append(&mut get_function_value(input, variables, value)?),
                            None => {
                                if name == &String::from("X") || name == &String::from("x") {
                                    by.push(Operator::Number { number: 1., x: 1, i: 0 });
                                } else {
                                    Err(format!("Unkwown variable {name}"))?
                                }
                            }
                        } 
                    }
                }
            },
            Operator::Equal => Err("Unautorized variable given to function")?,
            _ => by.push(ope.clone())
        }
    }

    Ok(from_postfix(&value.1).iter().fold(Vec::new(), |mut acc, ope| {
        match ope {
            Operator::Var(_) => acc.append(&mut by.clone()),
            _ => acc.push(ope.clone())
        }
        acc

    }))
}

fn change_known_variables(input: &Vec<Operator>, variables: &HashMap<String, (Option<String>, Vec<Operator>)>, v_name: Option<&String>) -> Result<Vec<Operator>, String> {
    let mut output = Vec::new();
    let mut iter = input.iter();
    while let Some(ope) = iter.next() {
        match ope {
            Operator::Var(name) => {
                match name.as_str() {
                    "i" | "I" => output.push(Operator::Number { number: 1., x: 0, i: 1 }),
                    _ => {
                        if let Some(v_name) = v_name {
                            if v_name == name {
                                output.push(ope.clone());
                                continue;
                            }
                        }
                        match variables.get(name) {
                            Some(value) => output.append(&mut get_function_value(&mut iter, variables, value)?),
                            None => {
                                if name == &String::from("X") || name == &String::from("x") {
                                    output.push(Operator::Number { number: 1., x: 1, i: 0 })
                                } else {
                                    Err(format!("Unkwown variable {name}"))?
                                }
                            }
                        } 
                    }
                }
            },
            _ => output.push(ope.clone()),
        }
    }
    shunting_yard(&mut output)
}

fn assign_minus(input: &mut Vec<Operator>) -> Result<(), String> {
    if input.is_empty() {
        Err("Empty input")?
    }
    let mut depth = 0;
    let mut natural_depth = 0;
    let mut ret = Vec::new();

    for index in 0..input.len() {
        if index > input.len() {
            continue
        }
        let end = if index + 3 > input.len() {
            input.len()
        } else {
            index + 3
        };
        match &input[index..end] {
            [a, b, c] => {
                match a {
                    Operator::Minus => {
                        match index {
                            0 => {
                                ret.append(&mut vec![Operator::Number { number: -1., x: 0, i: 0 }, Operator::Mult]);
                                continue
                            },
                            _ => if natural_depth == 0 {
                                while depth > 0 {
                                    ret.push(Operator::CloseParenthesis);
                                    depth -= 1;
                                }
                            }
                        }
                    }
                    Operator::Add | Operator::Mult | Operator::MatricialMult | Operator::Modulo | Operator::Div => {
                        if natural_depth == 0 {
                            while depth > 0 {
                                ret.push(Operator::CloseParenthesis);
                                depth -= 1;
                            }
                        }
                    }
                    Operator::OpenParenthesis => natural_depth += 1,
                    Operator::CloseParenthesis => natural_depth -= 1,
                    _ => {}
                }
                match b {
                    Operator::Equal => {
                        ret.push(a.clone());
                        while depth > 0 {
                            ret.push(Operator::CloseParenthesis);
                            depth -= 1;
                        }
                    }
                    Operator::Minus => {
                        match (a, c) {
                            (
                                Operator::Number { .. } | Operator::CloseParenthesis | Operator::Var(_) | Operator::Mat(_),
                                Operator::Number { .. } | Operator::OpenParenthesis | Operator::Var(_) | Operator::Mat(_)
                            ) => ret.push(a.clone()),
                            (
                                Operator::Add | Operator::Minus | Operator::Mult | Operator::Div | Operator::Modulo | Operator::MatricialMult | Operator::Power,
                                Operator::Add | Operator::Minus | Operator::Mult | Operator::Div | Operator::Modulo | Operator::MatricialMult | Operator::Power,
                            ) => Err("Too mutch operator next to each other")?,
                            (
                                Operator::Add | Operator::Minus | Operator::Mult | Operator::Div | Operator::Modulo | Operator::MatricialMult | Operator::Power | Operator::OpenParenthesis,
                                _,
                            ) => {
                                depth += 1;
                                ret.append(&mut vec![a.clone(), Operator::OpenParenthesis, Operator::Number { number: -1., x: 0, i: 0 }, Operator::Mult]);
                                input.remove(index + 1);
                            },
                            _ => Err("Unexpected token in input")?
                        }
                    },
                    _ => ret.push(a.clone())
                }
                
            }
            [a, _] | [a] => {
                match a {
                Operator::Minus => {
                    match index {
                        0 => {
                            ret.append(&mut vec![Operator::Number { number: -1., x: 0, i: 0 }, Operator::Mult]);
                            continue
                        },
                        _ => if natural_depth == 0 {
                            while depth > 0 {
                                ret.push(Operator::CloseParenthesis);
                                depth -= 1;
                            }
                            ret.push(a.clone())
                        }
                    }
                }
                Operator::Add | Operator::Mult | Operator::MatricialMult | Operator::Modulo | Operator::Div => {
                    if natural_depth == 0 {
                        while depth > 0 {
                            ret.push(Operator::CloseParenthesis);
                            depth -= 1;
                        }
                    }
                    ret.push(a.clone())
                }
                Operator::Equal => {
                    while depth > 0 {
                        ret.push(Operator::CloseParenthesis);
                        depth -= 1;
                    }
                    ret.push(a.clone());
                }
                Operator::Var(name) => match name.as_str() {
                    "?" => {
                        while depth > 0 {
                            ret.push(Operator::CloseParenthesis);
                            depth -= 1;
                        }
                        ret.push(a.clone());
                    }
                    _ => ret.push(a.clone())
                }
                _ => ret.push(a.clone())
            }
        },
            _ => {}
        }
    }

    while depth > 0 {
        ret.push(Operator::CloseParenthesis);
        depth -= 1;
    }
    
    *input = ret;
    Ok(())
}

fn intersect_with_mult(input: &mut Vec<Operator>) -> Result<(), String> {
    if input.is_empty() {
        Err("Empty input")?
    }
    let mut ret = Vec::new();
    for index in 0..input.len() {
        if index > input.len() {
            continue
        }
        let end = if index + 2 > input.len() {
            input.len()
        } else {
            index + 2
        };
        match &input[index..end] {
            [a, b] => {
                match (a, b) {
                    (
                        Operator::Number { .. } | Operator::Var(_) | Operator::Mat(_) | Operator::CloseParenthesis,
                        Operator::Number { .. } | Operator::Var(_) | Operator::Mat(_) | Operator::OpenParenthesis
                    ) => {
                        if let Operator::Var(name) = a {
                            if name == "?" {
                                ret.push(a.clone());
                                continue
                            }
                        } else if let Operator::Var(name) = b {
                            if name == "?" {
                                ret.push(a.clone());
                                continue
                            }
                        }
                        ret.push(a.clone());
                        ret.push(Operator::Mult);
                    }
                    _ => ret.push(a.clone()),
                }
            }
            [a] => ret.push(a.clone()),
            _ => {}
        }
    }
    *input = ret;
    Ok(())
}


pub fn parse_line(line: &str, variables: &HashMap<String, (Option<String>, Vec<Operator>)>) -> Result<ActionType, String> {
    let mut saved = String::default();
    let mut operators: Vec<Operator> = Vec::default();
    for c in line.chars() {
        if (c.is_numeric() || ".[];,".contains(c)) && saved.chars().all(|c| c.is_numeric() || ".[];, ".contains(c)) {
            saved.push(c);
        } else if !"+-()%^*=?/".contains(c) && c.is_alphabetic() && saved.chars().all(|c| !"+-()%^*=?/".contains(c) && c.is_alphabetic() || c == ' ') {
            saved.push(c);
        } else if c == ' ' {
            saved.push(c);
        } else {
            if saved.is_empty() && c == '*' {
                match operators.pop() {
                    Some(Operator::Mult) => {
                        operators.push(Operator::MatricialMult);
                        continue
                    },
                    Some(ope) => operators.push(ope),
                    None => {}
                }
            }

            if !saved.trim().is_empty() {
                operators.push(Operator::from_str(&saved)?);
                saved.clear();
            }
            operators.push(Operator::from_str(&String::from(c))?);
        }
    }
    if !saved.trim().is_empty() {
        operators.push(Operator::from_str(&saved)?);
        saved.clear();
    }
    operators
        .iter()
        .map(|ope| match ope {
            Operator::OpenParenthesis => 1,
            Operator::CloseParenthesis => -1,
            _ => 0
        })
        .fold(Ok(0), |acc, ope| {
            if acc? + ope < 0 {
                return Err("missmatched parenthesis")  
            }
            Ok(acc? + ope)
        })?;

    assign_minus(&mut operators)?;
    intersect_with_mult(&mut operators)?;

    let mut splitted: Vec<Vec<Operator>> = operators
        .split(|ope| ope == &Operator::Equal)
        .map(|ope| ope.to_vec())
        .collect();
    match splitted.len() {
        0..=1 => Err("expect 1 equal")?,
        2 => {}
        _ => Err("too mutch equal found")?
    }
    if splitted.get(1).unwrap().is_empty() {
        Err("Empty assignation")?
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
    })? {
        0 => { // assign
            let first_part = splitted.get(0).unwrap();
        
            match first_part.len() {
                0 => Err("Empty input")?,
                1 => { // Variable assignation
                    let first_part = splitted.get(0).unwrap();
                
                    match first_part.get(0).unwrap() {
                        Operator::Var(name) => {
                            if name == "i" || name == "I" {
                                Err("Cannot reassign i")?
                            }
                            Ok(ActionType::VarAssignation((
                                String::from(name),
                                change_known_variables(splitted.get(1).unwrap(), &variables, None)?
                            )))
                        },
                        _ => Err("Assignation is only available for variable or function. Eq:\n>> x = 50 - 8")?
                    }

                }
                4 => { // Function assignation
                    match &splitted.get(0).unwrap()[..] {
                        [Operator::Var(fn_name), Operator::OpenParenthesis, Operator::Var(var_name), Operator::CloseParenthesis] => {
                            if fn_name == "i" || fn_name == "I" {
                                Err("Cannot reassign i")?
                            } else if var_name == "i" || var_name == "I" {
                                Err("Cannot assign i as a variable")?
                            }
                            Ok(ActionType::FunAssignation((
                                String::from(fn_name),
                                String::from(var_name),
                                change_known_variables(splitted.get(1).unwrap(), &variables, Some(var_name))?
                                )))
                        }
                        _ => Err("Assignation is only available for variable or function. Eq:\n>> f(x) = 3 * x + 2")?
                    }
                }
                _ => Err("Assignation is only available for variable or function. Eq:\n>> x = 50 - 8\n>> f(x) = 3 * x + 2")?
            }
        },
        _ => { // Calculation
            match splitted.get_mut(1).unwrap().pop() {
                Some(Operator::Var(name)) => match name.as_str() {
                    "?" => {},
                    _ => Err("Expected ? at the end of the calculation part")?
                }
                _ => Err("Expected ? at the end of the calculation part")?
            }
            let ret = splitted
                .iter()
                .enumerate()
                .map(|(i, part)| {
                    match part.is_empty() && i == 0 {
                        true => Err(String::from("Empty first part")),
                        false => Ok(change_known_variables(part, &variables, None)?)
                    }
                })
                .collect::<Result<Vec<Vec<Operator>>, String>>()?;
            match (ret.get(0),ret.get(1)) {
                (Some(fst), Some(snd)) => return Ok(ActionType::Calculus((fst.clone(), snd.clone()))),
                _ => Err("Error while parsing calculus")?
            }
        }
    }
}


pub fn shunting_yard(input: &mut Vec<Operator>) -> Result<Vec<Operator>, String> {
    let mut output = Vec::new();
    let mut stack = Vec::new();
    input.reverse();
    'a: while let Some(operator) = input.pop() {
        match &operator {
            Operator::Var(name) => {
                match name.as_str() {
                    "?" => {
                        while let Some(stack_ope) = stack.pop() {
                            match stack_ope {
                                Operator::OpenParenthesis | Operator::CloseParenthesis => return Err(String::from("Missmatched parentesis")),
                                _ => output.push(stack_ope)
                            }
                        }
                        output.push(operator);
                    },
                    _ => output.push(operator)
                }
            }
            Operator::Number {..} | Operator::Mat(_) => output.push(operator),
            Operator::Equal => {
                while let Some(stack_ope) = stack.pop() {
                    match stack_ope {
                        Operator::OpenParenthesis | Operator::CloseParenthesis => return Err(String::from("Missmatched parentesis")),
                        _ => output.push(stack_ope)
                    }
                }
                output.push(Operator::Equal);
            }
            Operator::OpenParenthesis => stack.push(operator),
            Operator::CloseParenthesis => {
                while let Some(stack_ope) = stack.pop() {
                    match stack_ope {
                        Operator::OpenParenthesis => continue 'a,
                        Operator::CloseParenthesis => return Err(String::from("Missmatched parentesis")),
                        _ => output.push(stack_ope),
                    }
                }
                if stack.is_empty() {
                    return Err(String::from("Missmatched parentesis"))
                }
            }
            _ => {
                while let Some(stack_ope) = stack.pop() {
                    if stack_ope.get_precedence() > operator.get_precedence() || (stack_ope.get_precedence() == operator.get_precedence() && !operator.get_associativity()) {
                        output.push(stack_ope);
                    } else {
                        stack.push(stack_ope);
                        break;
                    }
                }
                stack.push(operator);
            }
        }
    }

    while let Some(stack_ope) = stack.pop() {
        match stack_ope {
            Operator::OpenParenthesis | Operator::CloseParenthesis => return Err(String::from("Missmatched parentesis")),
            _ => output.push(stack_ope)
        }
    }

    Ok(output)
}

#[cfg(test)]
mod minus {
    use super::*;

    #[test]
    fn all() {
        let mut x;

        // 2^-3 = 0 ? => 2^(-1 * 3) = 0 ?
        x = vec![ Operator::Number { number: 2., x: 0, i: 0 }, Operator::Power, Operator::Minus, Operator::Number { number: 3., x: 0, i: 0 }, Operator::Equal, Operator::Number { number: 0., x: 0, i: 0 }, Operator::Var(String::from("?"))];
        _ = assign_minus(&mut x);
        assert_eq!(x, vec![ Operator::Number { number: 2., x: 0, i: 0 }, Operator::Power, Operator::OpenParenthesis, Operator::Number { number: -1., x: 0, i: 0 }, Operator::Mult, Operator::Number { number: 3., x: 0, i: 0 }, Operator::CloseParenthesis, Operator::Equal, Operator::Number { number: 0., x: 0, i: 0 }, Operator::Var(String::from("?"))]);

        // -2^-3 =? => -1 * 2 ^ (-1 * 3) =?
        x = vec![Operator::Minus, Operator::Number { number: 2., x: 0, i: 0 }, Operator::Power, Operator::Minus, Operator::Number { number: 3., x: 0, i: 0 }, Operator::Equal, Operator::Var(String::from("?"))];
        _ = assign_minus(&mut x);
        assert_eq!(x, vec![Operator::Number { number: -1., x: 0, i: 0 }, Operator::Mult, Operator::Number { number: 2., x: 0, i: 0 }, Operator::Power, Operator::OpenParenthesis, Operator::Number { number: -1., x: 0, i: 0 }, Operator::Mult, Operator::Number { number: 3., x: 0, i: 0 }, Operator::CloseParenthesis, Operator::Equal, Operator::Var(String::from("?"))]);

        // 1 => 1
        x = vec![Operator::Number { number: 1., x: 0, i: 0 }];
        _ = assign_minus(&mut x);
        assert_eq!(x, vec![Operator::Number { number: 1., x: 0, i: 0 }]);

        // -1 => -1 * 1
        x = vec![Operator::Minus, Operator::Number { number: 1., x: 0, i: 0 }];
        _ = assign_minus(&mut x);
        assert_eq!(x, vec![Operator::Number { number: -1., x: 0, i: 0 }, Operator::Mult, Operator::Number { number: 1., x: 0, i: 0 }]);

        // 1-2 => 1-2
        x = vec![Operator::Number { number: 1., x: 0, i: 0 }, Operator::Minus, Operator::Number { number: 2., x: 0, i: 0 }];
        _ = assign_minus(&mut x);
        assert_eq!(x, vec![Operator::Number { number: 1., x: 0, i: 0 }, Operator::Minus, Operator::Number { number: 2., x: 0, i: 0 }]);

        // 1 + - 2 => 1 + (-1 * 2)
        x = vec![Operator::Number { number: 1., x: 0, i: 0 }, Operator::Add, Operator::Minus, Operator::Number { number: 2., x: 0, i: 0 }];
        _ = assign_minus(&mut x);
        assert_eq!(x, vec![Operator::Number { number: 1., x: 0, i: 0 }, Operator::Add, Operator::OpenParenthesis, Operator::Number { number: -1., x: 0, i: 0 }, Operator::Mult, Operator::Number { number: 2., x: 0, i: 0 }, Operator::CloseParenthesis]);

        // 1 + - 2 - 3 => 1 + (-1 * 2) - 3
        x = vec![Operator::Number { number: 1., x: 0, i: 0 }, Operator::Add, Operator::Minus, Operator::Number { number: 2., x: 0, i: 0 }, Operator::Minus, Operator::Number { number: 3., x: 0, i: 0 }];
        _ = assign_minus(&mut x);
        assert_eq!(x, vec![Operator::Number { number: 1., x: 0, i: 0 }, Operator::Add, Operator::OpenParenthesis, Operator::Number { number: -1., x: 0, i: 0 }, Operator::Mult, Operator::Number { number: 2., x: 0, i: 0 }, Operator::CloseParenthesis, Operator::Minus, Operator::Number { number: 3., x: 0, i: 0 }]);

        // 1 / - 2 =>  / (-1 * 2)
        x = vec![Operator::Number { number: 1., x: 0, i: 0 }, Operator::Div, Operator::Minus, Operator::Number { number: 2., x: 0, i: 0 }];
        _ = assign_minus(&mut x);
        assert_eq!(x, vec![Operator::Number { number: 1., x: 0, i: 0 }, Operator::Div, Operator::OpenParenthesis, Operator::Number { number: -1., x: 0, i: 0 }, Operator::Mult, Operator::Number { number: 2., x: 0, i: 0 }, Operator::CloseParenthesis]);

        // 1 ^ - 2 + 3 => 1 ^ (-1 * 2) + 3
        x = vec![Operator::Number { number: 1., x: 0, i: 0 }, Operator::Power, Operator::Minus, Operator::Number { number: 2., x: 0, i: 0 }, Operator::Add, Operator::Number { number: 3., x: 0, i: 0 }];
        _ = assign_minus(&mut x);
        assert_eq!(x, vec![Operator::Number { number: 1., x: 0, i: 0 }, Operator::Power, Operator::OpenParenthesis, Operator::Number { number: -1., x: 0, i: 0 }, Operator::Mult, Operator::Number { number: 2., x: 0, i: 0 }, Operator::CloseParenthesis, Operator::Add, Operator::Number { number: 3., x: 0, i: 0 }]);

        // (1 + 2) - (3 - 4) => (1 + 2) - (3 - 4)
        x = vec![Operator::OpenParenthesis, Operator::Number { number: 1., x: 0, i: 0 }, Operator::Add, Operator::Number { number: 2., x: 0, i: 0 }, Operator::CloseParenthesis, Operator::Minus, Operator::OpenParenthesis, Operator::Number { number: 3., x: 0, i: 0 }, Operator::Add, Operator::Number { number: 4., x: 0, i: 0 }, Operator::CloseParenthesis];
        _ = assign_minus(&mut x);
        assert_eq!(x, vec![Operator::OpenParenthesis, Operator::Number { number: 1., x: 0, i: 0 }, Operator::Add, Operator::Number { number: 2., x: 0, i: 0 }, Operator::CloseParenthesis, Operator::Minus, Operator::OpenParenthesis, Operator::Number { number: 3., x: 0, i: 0 }, Operator::Add, Operator::Number { number: 4., x: 0, i: 0 }, Operator::CloseParenthesis]);
    }
}

#[cfg(test)]
mod polish {
    use super::*;

    #[test]
    fn missmatched() {
        assert_eq!(shunting_yard(&mut vec![ // ( 2 + 2
            Operator::OpenParenthesis,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::Add,
            Operator::Number {number: 2., x: 0, i: 0 },
        ]).is_ok(), false);

        assert_eq!(shunting_yard(&mut vec![ // (2 + 2(
            Operator::OpenParenthesis,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::Add,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::OpenParenthesis,
        ]).is_ok(), false);

        assert_eq!(shunting_yard(&mut vec![ // ()2 + 2(
            Operator::OpenParenthesis,
            Operator::CloseParenthesis,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::Add,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::OpenParenthesis,
        ]).is_ok(), false);

        assert_eq!(shunting_yard(&mut vec![ // )2 + 2
            Operator::CloseParenthesis,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::Add,
            Operator::Number {number: 2., x: 0, i: 0 },
        ]).is_ok(), false);

        assert_eq!(shunting_yard(&mut vec![ // (2 + 2 = 2 - 2
            Operator::OpenParenthesis,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::Add,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::Equal,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::Minus,
            Operator::Number {number: 2., x: 0, i: 0 },
            ]).is_ok(), false);

        assert_eq!(shunting_yard(&mut vec![ // (2 + 2 = 2 - 2)
            Operator::OpenParenthesis,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::Add,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::Equal,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::Minus,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::CloseParenthesis,
            ]).is_ok(), false);

            assert_eq!(shunting_yard(&mut vec![ // (2 + 2 = (2 - 2)
                Operator::OpenParenthesis,
                Operator::Number {number: 2., x: 0, i: 0 },
                Operator::Add,
                Operator::Number {number: 2., x: 0, i: 0 },
                Operator::Equal,
                Operator::OpenParenthesis,
                Operator::Number {number: 2., x: 0, i: 0 },
                Operator::Minus,
                Operator::Number {number: 2., x: 0, i: 0 },
                Operator::CloseParenthesis,
                ]).is_ok(), false);
    }

    #[test]
    fn ok() {
        assert_eq!(shunting_yard(&mut vec![ // ( 1 + 2)
            Operator::OpenParenthesis,
            Operator::Number {number: 1., x: 0, i: 0 },
            Operator::Add,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::CloseParenthesis,
            ]),
            Ok(vec![
                Operator::Number {number: 1., x: 0, i: 0 },
                Operator::Number {number: 2., x: 0, i: 0 },
                Operator::Add,
            ]));

        assert_eq!(shunting_yard(&mut vec![ //  1 + 2 * 3
            Operator::Number {number: 1., x: 0, i: 0 },
            Operator::Add,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::Mult,
            Operator::Number {number: 3., x: 0, i: 0 },
            ]),
            Ok(vec![
                Operator::Number {number: 1., x: 0, i: 0 },
                Operator::Number {number: 2., x: 0, i: 0 },
                Operator::Number {number: 3., x: 0, i: 0 },
                Operator::Mult,
                Operator::Add,
            ]));

        assert_eq!(shunting_yard(&mut vec![ //  (1 + 2) * 3
            Operator::OpenParenthesis,
            Operator::Number {number: 1., x: 0, i: 0 },
            Operator::Add,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::CloseParenthesis,
            Operator::Mult,
            Operator::Number {number: 3., x: 0, i: 0 },
            ]),
            Ok(vec![
                Operator::Number {number: 1., x: 0, i: 0 },
                Operator::Number {number: 2., x: 0, i: 0 },
                Operator::Add,
                Operator::Number {number: 3., x: 0, i: 0 },
                Operator::Mult,
            ])
        );

        assert_eq!(shunting_yard(&mut vec![ //  a * ((b + c) * d / (e - f))
            Operator::Var(String::from("a")),
            Operator::Mult,
            Operator::OpenParenthesis,
            Operator::OpenParenthesis,
            Operator::Var(String::from("b")),
            Operator::Add,
            Operator::Var(String::from("c")),
            Operator::CloseParenthesis,
            Operator::Mult,
            Operator::Var(String::from("d")),
            Operator::Div,
            Operator::OpenParenthesis,
            Operator::Var(String::from("e")),
            Operator::Minus,
            Operator::Var(String::from("f")),
            Operator::CloseParenthesis,
            Operator::CloseParenthesis,
            ]),
            Ok(vec![
                Operator::Var(String::from("a")),
                Operator::Var(String::from("b")),
                Operator::Var(String::from("c")),
                Operator::Add,
                Operator::Var(String::from("d")),
                Operator::Mult,
                Operator::Var(String::from("e")),
                Operator::Var(String::from("f")),
                Operator::Minus,
                Operator::Div,
                Operator::Mult,
            ])
        );

        assert_eq!(shunting_yard(&mut vec![ //  6 * ((1 + 2) * 3 ^ 7 / (4 - 5))
            Operator::Number {number: 6., x: 0, i: 0 },
            Operator::Mult,
            Operator::OpenParenthesis,
            Operator::OpenParenthesis,
            Operator::Number {number: 1., x: 0, i: 0 },
            Operator::Add,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::CloseParenthesis,
            Operator::Mult,
            Operator::Number {number: 3., x: 0, i: 0 },
            Operator::Power,
            Operator::Number {number: 7., x: 0, i: 0 },
            Operator::Div,
            Operator::OpenParenthesis,
            Operator::Number {number: 4., x: 0, i: 0 },
            Operator::Minus,
            Operator::Number {number: 5., x: 0, i: 0 },
            Operator::CloseParenthesis,
            Operator::CloseParenthesis,
            ]),
            Ok(vec![
                Operator::Number {number: 6., x: 0, i: 0 },
                Operator::Number {number: 1., x: 0, i: 0 },
                Operator::Number {number: 2., x: 0, i: 0 },
                Operator::Add,
                Operator::Number {number: 3., x: 0, i: 0 },
                Operator::Number {number: 7., x: 0, i: 0 },
                Operator::Power,
                Operator::Mult,
                Operator::Number {number: 4., x: 0, i: 0 },
                Operator::Number {number: 5., x: 0, i: 0 },
                Operator::Minus,
                Operator::Div,
                Operator::Mult,
            ])
        );

        assert_eq!(shunting_yard(&mut vec![ //  6 * ((1 + 2) * 3 ^ 7 / (4 - 5)) = 8 - 9 * 10
            Operator::Number {number: 6., x: 0, i: 0 },
            Operator::Mult,
            Operator::OpenParenthesis,
            Operator::OpenParenthesis,
            Operator::Number {number: 1., x: 0, i: 0 },
            Operator::Add,
            Operator::Number {number: 2., x: 0, i: 0 },
            Operator::CloseParenthesis,
            Operator::Mult,
            Operator::Number {number: 3., x: 0, i: 0 },
            Operator::Power,
            Operator::Number {number: 7., x: 0, i: 0 },
            Operator::Div,
            Operator::OpenParenthesis,
            Operator::Number {number: 4., x: 0, i: 0 },
            Operator::Minus,
            Operator::Number {number: 5., x: 0, i: 0 },
            Operator::CloseParenthesis,
            Operator::CloseParenthesis,
            Operator::Equal,
            Operator::Number {number: 8., x: 0, i: 0 },
            Operator::Minus,
            Operator::Number {number: 9., x: 0, i: 0 },
            Operator::Mult,
            Operator::Number {number: 10., x: 0, i: 0 },
            ]),
            Ok(vec![
                Operator::Number {number: 6., x: 0, i: 0 },
                Operator::Number {number: 1., x: 0, i: 0 },
                Operator::Number {number: 2., x: 0, i: 0 },
                Operator::Add,
                Operator::Number {number: 3., x: 0, i: 0 },
                Operator::Number {number: 7., x: 0, i: 0 },
                Operator::Power,
                Operator::Mult,
                Operator::Number {number: 4., x: 0, i: 0 },
                Operator::Number {number: 5., x: 0, i: 0 },
                Operator::Minus,
                Operator::Div,
                Operator::Mult,
                Operator::Equal,
                Operator::Number {number: 8., x: 0, i: 0 },
                Operator::Number {number: 9., x: 0, i: 0 },
                Operator::Number {number: 10., x: 0, i: 0 },
                Operator::Mult,
                Operator::Minus,
            ])
        );

    }
}