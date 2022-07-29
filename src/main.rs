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


#[derive(Clone, Debug)]
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
    Div
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
        let mat: Vec<Vec<Operator>> = Vec::default();
        let mut depth = 0;
        for c in value.split_inclusive(|c| ";[],".contains(c)) {
            match c.trim() {
                "[" => depth += 1,
                "]" => depth -= 1,
                _ => {
                    let mut string = String::from(c);
                    if let Some(last) = string.pop() {
                        if !string.trim().is_empty() {
                            match last {
                                ']' => depth -= 1,
                                ';' | '[' => {
                                    println!("Error found {last}");
                                    return None
                                },
                                _ => {}
                            }
                            println!("{string:?} {last}");
                            if let Ok(nb) = string.trim().parse::<f64>() {
                                println!("{:?}", Operator::Number(nb));
                            } else if let Ok(ope) = Operator::from_str(&string) {
                                println!("{ope:?}");
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
        None
    }
}

fn parse_line(line: &str) -> Result<(), String>{
    let mut saved = String::default();
    let mut operators: Vec<Operator> = Vec::default();
    for c in line.chars() {
        if !"+-()%^".contains(c) || String::from(c).parse::<u8>().is_ok() {
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