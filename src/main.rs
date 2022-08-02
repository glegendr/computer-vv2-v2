use rustyline::Editor;

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
                        println!("{operators:?}");
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

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    Var(String),
    Number(f64),
    // Function {name: String, value: String},
    Add,
    Minus,
    Mult,
    MatricialMult,
    Modulo,
    Power,
    OpenParenthesis,
    CloseParenthesis,
    Div,
    Mat(Vec<Vec<Operator>>),
    Equal
}

impl Operator {

    fn get_precedence(&self) -> u8 {
        match self {
            Operator::Add | Operator::Minus => 2,
            Operator::Mult | Operator::Div => 3,
            Operator::Power => 4,
            Operator::CloseParenthesis | Operator::OpenParenthesis => 1,
            _ => unreachable!()
        }
    }

    fn get_associativity(&self) -> bool {
        match self {
            Operator::Power => true,
            _ => false
        }
    }

    fn from_str(value: &str) -> Result<Self, String> {
        match value.trim() {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Minus),
            "*" => Ok(Self::Mult),
            "**" => Ok(Self::MatricialMult),
            "%" => Ok(Self::Modulo),
            "^" => Ok(Self::Power),
            "(" => Ok(Self::OpenParenthesis),
            ")" => Ok(Self::CloseParenthesis),
            "/" => Ok(Self::Div),
            "=" => Ok(Self::Equal),
            value => {
                if let Some(ope) = Operator::parse_mat(value) {
                    return Ok(ope)
                }
                match value.parse::<f64>() {
                    Ok(value_nb) => Ok(Self::Number(value_nb)),
                    _ => {
                        if value.contains(|c: char| "+-*%^,; .[]/".contains(c) || String::from(c).parse::<i32>().is_ok()) {
                            Err(format!("{value} is invalid name for a variable"))
                        } else {
                            Ok(Self::Var(String::from(value)))
                        }
                    }
                }
            }
        }
    }

    fn parse_mat(value: &str) -> Option<Self> {
        let mut mat: Vec<Vec<Operator>> = Vec::default();
        let mut depth = 0;
        let mut mat_id  = 0;
        let mut comma_id = 0;
        for c in value.split_inclusive(|c| ";[],".contains(c)) {
            match c.trim() {
                "[" => depth += 1,
                "]" => {
                    mat_id += 1;
                    depth -= 1
                },
                ";" => {
                    if depth != 1 {
                        return None
                    }
                    comma_id += 1
                },
                _ => {
                    let mut string = String::from(c);
                    if let Some(last) = string.pop() {
                        if !string.trim().is_empty() {
                            if let Ok(nb) = string.trim().parse::<f64>() {
                                if depth != 2 || mat_id != comma_id {
                                    return None
                                }
                                if let Some(row) = mat.get_mut(mat_id) {
                                    row.push(Operator::Number(nb))
                                } else {
                                    mat.push(vec![Operator::Number(nb)])
                                }
                            } else if let Ok(ope) = Operator::from_str(&string) {
                                if depth != 2 || mat_id != comma_id {
                                    return None
                                }
                                if let Some(row) = mat.get_mut(mat_id) {
                                    row.push(ope)
                                } else {
                                    mat.push(vec![ope])
                                }
                            } else {
                                return None
                            }
                            match last {
                                ']' => {
                                    mat_id += 1;
                                    depth -= 1
                                },
                                ';' | '[' => {
                                    return None
                                },
                                _ => {}
                            }
                        } else if let Ok(nb) = string.trim().parse::<f64>() {
                            println!("{:?}", Operator::Number(nb));
                        }
                    }
                }
            };
            if depth < 0 ||  depth > 2 {
                return None
            }
        }
        if depth != 0 {
            return None
        }
        if mat.is_empty() || mat.iter().fold((false, -1 ), |acc, row| (acc.0 || (row.len() as i32 != acc.1 && acc.1 != -1), row.len() as i32)).0 {
            return None
        }
        Some(Operator::Mat(mat))
    }
}

fn parse_line(line: &str) -> Result<Vec<Operator>, String> {
    let mut saved = String::default();
    let mut operators: Vec<Operator> = Vec::default();
    for c in line.chars() {
        if !"+-()%^*=".contains(c) || String::from(c).parse::<u8>().is_ok() {
            saved.push(c);
        } else {
            if !saved.is_empty() {
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
        match operator {
            Operator::Number(_) | Operator::Mat(_)  | Operator::Var(_) => output.push(operator),
            Operator::Equal => {
                while let Some(stack_ope) = stack.pop() {
                    match stack_ope {
                        Operator::OpenParenthesis | Operator::CloseParenthesis => return Err(String::from("Missmatched parentesis 02")),
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
                        Operator::CloseParenthesis => return Err(String::from("Missmatched parentesis 00")),
                        _ => output.push(stack_ope),
                    }
                }
                if stack.is_empty() {
                    return Err(String::from("Missmatched parentesis 01"))
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
            Operator::OpenParenthesis | Operator::CloseParenthesis => return Err(String::from("Missmatched parentesis 02")),
            _ => output.push(stack_ope)
        }
    }

    Ok(output)
}

#[cfg(test)]
mod matrices {
    use super::*;

    #[test]
    fn good_mat() {
        assert_eq!(Operator::parse_mat("[[1 ,2  ,3   ];[ 1, 2,3 ]]"), Some(Operator::Mat(vec![
            vec![
                Operator::Number(1.),
                Operator::Number(2.),
                Operator::Number(3.)
            ], vec![
                Operator::Number(1.),
                Operator::Number(2.),
                Operator::Number(3.)
            ]
            ])));
            assert_eq!(Operator::parse_mat("[[                 1,2]]"), Some(Operator::Mat(vec![
                vec![
                    Operator::Number(1.),
                    Operator::Number(2.)
                ]
                ])));
            assert_eq!(Operator::parse_mat("[[\t\t\nsalut,2]]"), Some(Operator::Mat(vec![
                vec![
                    Operator::Var(String::from("salut")),
                    Operator::Number(2.)
                ]
                ])));
            assert_eq!(Operator::parse_mat("[[\t\t\n1,2]]"), Some(Operator::Mat(vec![
                vec![
                    Operator::Number(1.),
                    Operator::Number(2.)
                ]
                ])));
            assert_eq!(Operator::parse_mat("[[1];[2];[3];[4];[5]]"), Some(Operator::Mat(vec![
                vec![Operator::Number(1.)],
                vec![Operator::Number(2.)],
                vec![Operator::Number(3.)],
                vec![Operator::Number(4.)],
                vec![Operator::Number(5.)],
                ])));
    }
    
    #[test]
    fn bad_size() {
        assert_eq!(Operator::parse_mat("[[1,2,3];[1,2]]"), None);
        assert_eq!(Operator::parse_mat("[[1,2,3];[1,2,3, 4];[1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[[1,2,3];[];[2,3, 4];[1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[[1];[2];[3];[4];[]]]"), None);
        assert_eq!(Operator::parse_mat("[[1];[2];[3];[4];[5, 6]]]"), None);
    }

    #[test]
    fn double_point_virgule() {
        assert_eq!(Operator::parse_mat("[[1,2,3];;[1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[;[1,2,3][1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[[1,2,3][1,2, 3];]"), None);
        assert_eq!(Operator::parse_mat("[[2,3,4;][1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[[3,4,5][;1,2, 3]]"), None);
    }

    #[test]
    fn no_point_virgule() {
        assert_eq!(Operator::parse_mat("[[1,2,3][1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[[1,2,3];[1,2,3][1,2, 3]]"), None);
    }

    #[test]
    fn bad_depth() {
        assert_eq!(Operator::parse_mat("[[1,2,3];[[],2,3, 4];[1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[1,2,3]"), None);
    }

    #[test]
    fn operator_inside() {
        assert_eq!(Operator::parse_mat("[[1,2,3];[2,3 + 2, 4];[1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[[1,2,3];[2,3 * 2, 4];[1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[[1,2,3];[2,3 / 2, 4];[1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[[1,2,3];[2,3 % 2, 4];[1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[[1,2,3];[2,3 ^ 2, 4];[1,2, 3]]"), None);
    }

    #[test]
    fn bad_separator() {
        assert_eq!(Operator::parse_mat("[[1,2,3],[2,3, 4],[1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[[1,2,3];[2;3, 4];[1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[[1,2,3];[2.3 . 4];[1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[[1,2,3];[2 # 3 # 4];[1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[[1,2,3];[2 ! 3 ! 4];[1,2, 3]]"), None);
        assert_eq!(Operator::parse_mat("[[1,2,3];[2 \n 3 , 4];[1,2, 3]]"), None);
    }

    #[test]
    fn empty() {
        assert_eq!(Operator::parse_mat(""), None);
        assert_eq!(Operator::parse_mat("[]"), None);
        assert_eq!(Operator::parse_mat("[[]]"), None);
        assert_eq!(Operator::parse_mat("[[];[]]"), None);
        assert_eq!(Operator::parse_mat("[[];[];[]]"), None);
    }

}

#[cfg(test)]
mod polish {
    use super::*;

    #[test]
    fn missmatched() {
        assert_eq!(shunting_yard(&mut vec![ // ( 2 + 2
            Operator::OpenParenthesis,
            Operator::Number(2.),
            Operator::Add,
            Operator::Number(2.),
        ]).is_ok(), false);

        assert_eq!(shunting_yard(&mut vec![ // (2 + 2(
            Operator::OpenParenthesis,
            Operator::Number(2.),
            Operator::Add,
            Operator::Number(2.),
            Operator::OpenParenthesis,
        ]).is_ok(), false);

        assert_eq!(shunting_yard(&mut vec![ // ()2 + 2(
            Operator::OpenParenthesis,
            Operator::CloseParenthesis,
            Operator::Number(2.),
            Operator::Add,
            Operator::Number(2.),
            Operator::OpenParenthesis,
        ]).is_ok(), false);

        assert_eq!(shunting_yard(&mut vec![ // )2 + 2
            Operator::CloseParenthesis,
            Operator::Number(2.),
            Operator::Add,
            Operator::Number(2.),
        ]).is_ok(), false);

        assert_eq!(shunting_yard(&mut vec![ // (2 + 2 = 2 - 2
            Operator::OpenParenthesis,
            Operator::Number(2.),
            Operator::Add,
            Operator::Number(2.),
            Operator::Equal,
            Operator::Number(2.),
            Operator::Minus,
            Operator::Number(2.),
            ]).is_ok(), false);

        assert_eq!(shunting_yard(&mut vec![ // (2 + 2 = 2 - 2)
            Operator::OpenParenthesis,
            Operator::Number(2.),
            Operator::Add,
            Operator::Number(2.),
            Operator::Equal,
            Operator::Number(2.),
            Operator::Minus,
            Operator::Number(2.),
            Operator::CloseParenthesis,
            ]).is_ok(), false);

            assert_eq!(shunting_yard(&mut vec![ // (2 + 2 = (2 - 2)
                Operator::OpenParenthesis,
                Operator::Number(2.),
                Operator::Add,
                Operator::Number(2.),
                Operator::Equal,
                Operator::OpenParenthesis,
                Operator::Number(2.),
                Operator::Minus,
                Operator::Number(2.),
                Operator::CloseParenthesis,
                ]).is_ok(), false);
    }

    #[test]
    fn ok() {
        assert_eq!(shunting_yard(&mut vec![ // ( 1 + 2)
            Operator::OpenParenthesis,
            Operator::Number(1.),
            Operator::Add,
            Operator::Number(2.),
            Operator::CloseParenthesis,
            ]),
            Ok(vec![
                Operator::Number(1.),
                Operator::Number(2.),
                Operator::Add,
            ]));

        assert_eq!(shunting_yard(&mut vec![ //  1 + 2 * 3
            Operator::Number(1.),
            Operator::Add,
            Operator::Number(2.),
            Operator::Mult,
            Operator::Number(3.),
            ]),
            Ok(vec![
                Operator::Number(1.),
                Operator::Number(2.),
                Operator::Number(3.),
                Operator::Mult,
                Operator::Add,
            ]));

        assert_eq!(shunting_yard(&mut vec![ //  (1 + 2) * 3
            Operator::OpenParenthesis,
            Operator::Number(1.),
            Operator::Add,
            Operator::Number(2.),
            Operator::CloseParenthesis,
            Operator::Mult,
            Operator::Number(3.),
            ]),
            Ok(vec![
                Operator::Number(1.),
                Operator::Number(2.),
                Operator::Add,
                Operator::Number(3.),
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
            Operator::Number(6.),
            Operator::Mult,
            Operator::OpenParenthesis,
            Operator::OpenParenthesis,
            Operator::Number(1.),
            Operator::Add,
            Operator::Number(2.),
            Operator::CloseParenthesis,
            Operator::Mult,
            Operator::Number(3.),
            Operator::Power,
            Operator::Number(7.),
            Operator::Div,
            Operator::OpenParenthesis,
            Operator::Number(4.),
            Operator::Minus,
            Operator::Number(5.),
            Operator::CloseParenthesis,
            Operator::CloseParenthesis,
            ]),
            Ok(vec![
                Operator::Number(6.),
                Operator::Number(1.),
                Operator::Number(2.),
                Operator::Add,
                Operator::Number(3.),
                Operator::Number(7.),
                Operator::Power,
                Operator::Mult,
                Operator::Number(4.),
                Operator::Number(5.),
                Operator::Minus,
                Operator::Div,
                Operator::Mult,
            ])
        );

        assert_eq!(shunting_yard(&mut vec![ //  6 * ((1 + 2) * 3 ^ 7 / (4 - 5)) = 8 - 9 * 10
            Operator::Number(6.),
            Operator::Mult,
            Operator::OpenParenthesis,
            Operator::OpenParenthesis,
            Operator::Number(1.),
            Operator::Add,
            Operator::Number(2.),
            Operator::CloseParenthesis,
            Operator::Mult,
            Operator::Number(3.),
            Operator::Power,
            Operator::Number(7.),
            Operator::Div,
            Operator::OpenParenthesis,
            Operator::Number(4.),
            Operator::Minus,
            Operator::Number(5.),
            Operator::CloseParenthesis,
            Operator::CloseParenthesis,
            Operator::Equal,
            Operator::Number(8.),
            Operator::Minus,
            Operator::Number(9.),
            Operator::Mult,
            Operator::Number(10.),
            ]),
            Ok(vec![
                Operator::Number(6.),
                Operator::Number(1.),
                Operator::Number(2.),
                Operator::Add,
                Operator::Number(3.),
                Operator::Number(7.),
                Operator::Power,
                Operator::Mult,
                Operator::Number(4.),
                Operator::Number(5.),
                Operator::Minus,
                Operator::Div,
                Operator::Mult,
                Operator::Equal,
                Operator::Number(8.),
                Operator::Number(9.),
                Operator::Number(10.),
                Operator::Mult,
                Operator::Minus,
            ])
        );

    }
}