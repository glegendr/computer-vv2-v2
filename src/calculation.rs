use crate::operator::Operator;

pub fn calc(input: &Vec<Operator>) -> Result<Vec<Operator>, String> {
    let mut changed = false;
    let mut ret: Vec<Operator> = Vec::new();

    for ope in input {
        match ope {
            Operator::Var(_name) => unimplemented!(),
            Operator::Number {..} | Operator::Mat(_) => ret.push(ope.clone()),
            Operator::Add | Operator::Minus | Operator::Mult | Operator::Modulo | Operator::MatricialMult | Operator::Power | Operator::Div => {
                match ret.pop() {
                    Some(last) => {
                        match ret.pop() {
                            Some(last_last) => {
                                if let Some(calculated) = ope.calc(&last_last, &last) {
                                    ret.push(calculated);
                                    changed = true;
                                } else {
                                    ret.push(last_last);
                                    ret.push(last);
                                    ret.push(ope.clone());
                                }
                            },
                            _ => return Err(String::from("wrong input"))
                        }
                    }
                    _ => return Err(String::from("wrong input"))
                } 
            }
            _ => return Err(String::from("unexpected token wile calculation"))   
        }
    }
    
    // println!("calc: {ret:?}");

    if changed {
        return calc(&ret)
    }
    Ok(ret)
}