use crate::operator::Operator;

// TODO 3 * -1
pub fn parse_line(line: &str) -> Result<Vec<Operator>, String> {
    let mut saved = String::default();
    let mut operators: Vec<Operator> = Vec::default();
    for c in line.chars() {
        if !"+-()%^*=?/".contains(c) || String::from(c).parse::<u8>().is_ok() {
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

    shunting_yard(&mut operators)
}


fn shunting_yard(input: &mut Vec<Operator>) -> Result<Vec<Operator>, String> {
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