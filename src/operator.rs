#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Var(String),
    Number { number: f64, x: i32, i: i32 },
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
    pub fn get_precedence(&self) -> u8 {
        match self {
            Operator::Add | Operator::Minus => 2,
            Operator::Mult | Operator::Div => 3,
            Operator::Power => 4,
            Operator::CloseParenthesis | Operator::OpenParenthesis => 1,
            _ => unreachable!()
        }
    }

    pub fn get_associativity(&self) -> bool {
        match self {
            Operator::Power => true,
            _ => false
        }
    }

    pub fn from_str(value: &str) -> Result<Self, String> {
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
                    Ok(value_nb) => Ok(Self::Number { number: value_nb, x: 0, i: 0 }),
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
                                    row.push(Operator::Number { number: nb, x: 0, i: 0 })
                                } else {
                                    mat.push(vec![Operator::Number { number: nb, x: 0, i: 0 }])
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
                            println!("{:?}", Operator::Number { number: nb, x: 0, i: 0 });
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

    pub fn calc(&self, a: &Operator, b: &Operator) -> Option<Operator> {
        match self {
            Self::Add => {
                match a {
                    Operator::Number { number: number_a, x: x_a, i: i_a } => {
                      match b {
                        Operator::Number { number: number_b, x: x_b, i: i_b } => {
                            if x_a == x_b && i_a % 2 == i_b % 2 {
                                let mult_a = if i_a % 4 < 2 {
                                    1.
                                }  else {
                                    -1.
                                };
                                let mult_b = if i_b % 4 < 2 {
                                    1.
                                }  else {
                                    -1.
                                };
                                return Some(Operator::Number { number: number_a * mult_a + number_b * mult_b, x: *x_a, i: i_a % 2 })
                            } else if number_a == &0. {
                                let mult_b = if i_b % 4 < 2 {
                                    1.
                                }  else {
                                    -1.
                                };
                                return Some(Operator::Number { number: number_b * mult_b, x: *x_b, i: i_b % 2 })
                            } else if number_b == &0. {
                                let mult_a = if i_a % 4 < 2 {
                                    1.
                                }  else {
                                    -1.
                                };
                                return Some(Operator::Number { number: number_a * mult_a, x: *x_a, i: i_a % 2 })

                            }
                            return None
                        }
                        _ => unimplemented!(),
                      }  
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
        None
    }
}


#[cfg(test)]
mod matrices {
    use super::*;

    #[test]
    fn good_mat() {
        assert_eq!(Operator::parse_mat("[[1 ,2  ,3   ];[ 1, 2,3 ]]"), Some(Operator::Mat(vec![
            vec![
                Operator::Number {number: 1., x: 0, i: 0 },
                Operator::Number {number: 2., x: 0, i: 0 },
                Operator::Number {number: 3., x: 0, i: 0 }
            ], vec![
                Operator::Number {number: 1., x: 0, i: 0 },
                Operator::Number {number: 2., x: 0, i: 0 },
                Operator::Number {number: 3., x: 0, i: 0 }
            ]
            ])));
            assert_eq!(Operator::parse_mat("[[                 1,2]]"), Some(Operator::Mat(vec![
                vec![
                    Operator::Number {number: 1., x: 0, i: 0 },
                    Operator::Number {number: 2., x: 0, i: 0 }
                ]
                ])));
            assert_eq!(Operator::parse_mat("[[\t\t\nsalut,2]]"), Some(Operator::Mat(vec![
                vec![
                    Operator::Var(String::from("salut")),
                    Operator::Number {number: 2., x: 0, i: 0 }
                ]
                ])));
            assert_eq!(Operator::parse_mat("[[\t\t\n1,2]]"), Some(Operator::Mat(vec![
                vec![
                    Operator::Number {number: 1., x: 0, i: 0 },
                    Operator::Number {number: 2., x: 0, i: 0 }
                ]
                ])));
            assert_eq!(Operator::parse_mat("[[1];[2];[3];[4];[5]]"), Some(Operator::Mat(vec![
                vec![Operator::Number {number: 1., x: 0, i: 0 }],
                vec![Operator::Number {number: 2., x: 0, i: 0 }],
                vec![Operator::Number {number: 3., x: 0, i: 0 }],
                vec![Operator::Number {number: 4., x: 0, i: 0 }],
                vec![Operator::Number {number: 5., x: 0, i: 0 }],
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
mod calc {

    use super::*;

    #[test]
    fn add_simple() {
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 0., x: 0, i: 0}, &Operator::Number { number: 4., x: 0, i: 0}), Some(Operator::Number { number: 4., x: 0, i: 0}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 0, i: 0}, &Operator::Number { number: 2., x: 0, i: 0}), Some(Operator::Number { number: 5., x: 0, i: 0}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 0, i: 0}, &Operator::Number { number: -2., x: 0, i: 0}), Some(Operator::Number { number: 1., x: 0, i: 0}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: -3., x: 0, i: 0}, &Operator::Number { number: -2., x: 0, i: 0}), Some(Operator::Number { number: -5., x: 0, i: 0}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: -3., x: 0, i: 0}, &Operator::Number { number: 1., x: 0, i: 0}), Some(Operator::Number { number: -2., x: 0, i: 0}));
    }

    #[test]
    fn add_x() {
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 1, i: 0}, &Operator::Number { number: 4., x: 1, i: 0}), Some(Operator::Number { number: 7., x: 1, i: 0}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 2, i: 0}, &Operator::Number { number: 4., x: 2, i: 0}), Some(Operator::Number { number: 7., x: 2, i: 0}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 2, i: 0}, &Operator::Number { number: -4., x: 2, i: 0}), Some(Operator::Number { number: -1., x: 2, i: 0}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 99, i: 0}, &Operator::Number { number: -4., x: 99, i: 0}), Some(Operator::Number { number: -1., x: 99, i: 0}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 99, i: 0}, &Operator::Number { number: -4., x: 1, i: 0}), None);
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 99, i: 0}, &Operator::Number { number: -4., x: 98, i: 0}), None);
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 99, i: 0}, &Operator::Number { number: 3., x: 98, i: 0}), None);
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: -1, i: 0}, &Operator::Number { number: 3., x: -2, i: 0}), None);
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: -1, i: 0}, &Operator::Number { number: 3., x: -1, i: 0}), Some(Operator::Number { number: 6., x: -1, i: 0}));
    }

    #[test]
    fn add_i() {
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 0, i: 1}, &Operator::Number { number: 4., x: 0, i: 1}), Some(Operator::Number { number: 7., x: 0, i: 1}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 0, i: 1}, &Operator::Number { number: 4., x: 0, i: 2}), None);
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 0, i: 0}, &Operator::Number { number: 4., x: 0, i: 2}), Some(Operator::Number { number: -1., x: 0, i: 0}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 0, i: 0}, &Operator::Number { number: 4., x: 0, i: 98}), Some(Operator::Number { number: -1., x: 0, i: 0}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 0, i: 1}, &Operator::Number { number: 4., x: 0, i: 3}), Some(Operator::Number { number: -1., x: 0, i: 1}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 0., x: 0, i: 1}, &Operator::Number { number: 4., x: 0, i: 3}), Some(Operator::Number { number: -4., x: 0, i: 1}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 0., x: 0, i: 0}, &Operator::Number { number: 4., x: 0, i: 3}), Some(Operator::Number { number: -4., x: 0, i: 1}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 4., x: 0, i: 3}, &Operator::Number { number: 0., x: 0, i: 3}), Some(Operator::Number { number: -4., x: 0, i: 1}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 4., x: 0, i: 3}, &Operator::Number { number: 0., x: 0, i: 0}), Some(Operator::Number { number: -4., x: 0, i: 1}));
    }

    #[test]
    fn add_i_x() {
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 1, i: 1}, &Operator::Number { number: 4., x: 1, i: 3}), Some(Operator::Number { number: -1., x: 1, i: 1}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 1, i: 1}, &Operator::Number { number: 4., x: 0, i: 3}), None);
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 2, i: 1}, &Operator::Number { number: 4., x: 2, i: 3}), Some(Operator::Number { number: -1., x: 2, i: 1}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 2, i: 2}, &Operator::Number { number: 4., x: 2, i: 0}), Some(Operator::Number { number: 1., x: 2, i: 0}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 2, i: 4}, &Operator::Number { number: 4., x: 2, i: 0}), Some(Operator::Number { number: 7., x: 2, i: 0}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 2, i: 4}, &Operator::Number { number: 4., x: 2, i: 4}), Some(Operator::Number { number: 7., x: 2, i: 0}));
        assert_eq!(Operator::Add.calc(&Operator::Number { number: 3., x: 2, i: 4}, &Operator::Number { number: 4., x: 2, i: 0}), Some(Operator::Number { number: 7., x: 2, i: 0}));
    }


}