
use crate::{operator::Operator};

pub fn to_printable_string(value: &Vec<Operator>) -> String {
    let new_value = from_postfix(value);
    let mut ret = String::new();
    for ope in new_value {
        ret = format!("{ret}{ope} ");
    }
    ret
}

pub fn from_postfix(value: &Vec<Operator>) -> Vec<Operator> {
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