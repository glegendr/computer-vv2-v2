use crate::{operator::Operator, btree::BTree};

pub fn calc(input: &Vec<Operator>) -> Result<BTree, String> {
    // let mut changed = false;
    let tree = BTree::from_vec(&input)?;
    // println!("BEFORE {tree}");
    // tree.print();
    tree.eval()
    // eval.print();
    // for ope in input {
    //     match ope {
    //         Operator::Var(name) => return Err(format!("Unknown variable {name}")),
    //         Operator::Number {..} | Operator::Mat(_) => ret.push(ope.clone()),
    //         Operator::Add | Operator::Minus | Operator::Mult | Operator::Modulo | Operator::MatricialMult | Operator::Power | Operator::Div => {
    //             match ret.pop() {
    //                 Some(last) => {
    //                     match ret.pop() {
    //                         Some(last_last) => {
    //                             if let Some(calculated) = ope.calc(&last_last, &last) {
    //                                 ret.push(calculated);
    //                                 changed = true;
    //                             } else {
    //                                 ret.push(last_last);
    //                                 ret.push(last);
    //                                 ret.push(ope.clone());
    //                             }
    //                         },
    //                         _ => return Err(String::from("wrong input"))
    //                     }
    //                 }
    //                 _ => return Err(String::from("wrong input"))
    //             } 
    //         }
    //         _ => return Err(String::from("unexpected token wile calculation"))   
    //     }
    // }
    

    // if changed {
    //     return calc(&ret)
    // }
    // println!("calc: {ret:?}");
}