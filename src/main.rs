use rustyline::Editor;

fn main() -> rustyline::Result<()> {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new()?;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if let Err(e) = parse_line(line.as_str()) {
                    println!("{e}");
                }
            },
            Err(err) => {
                println!("Error: {:?}", err);
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
    Add,
    Minus,
    Mult,
    MatricialMult,
    Modulo,
    Power,
    OpenParenthesis,
    CloseParenthesis,
    Div,
    Mat(Vec<Vec<Operator>>)
}

impl Operator {
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
                            println!("{string:?} {last}");
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

fn parse_line(line: &str) -> Result<(), String>{
    let mut saved = String::default();
    let mut operators: Vec<Operator> = Vec::default();
    for c in line.chars() {
        if !"+-()%^*".contains(c) || String::from(c).parse::<u8>().is_ok() {
            saved.push(c);
        } else {
            operators.push(Operator::from_str(&saved)?);
            saved.clear();
            operators.push(Operator::from_str(&String::from(c))?);
        }
    }
    if !saved.trim().is_empty() {
        operators.push(Operator::from_str(&saved)?);
        saved.clear();
    }
    println!("{:?}", operators);
    Ok(())
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