
use std::fmt;
use crate::operator::Operator;
/* ---------- BTREE ---------- */
#[derive(Debug, Clone, PartialEq)]
pub struct BTree {
    pub c1: Option<Box<BTree>>,
    pub c2: Option<Box<BTree>>,
    pub node: Operator
}

impl fmt::Display for BTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{}", self.to_string()))
    }
}

impl BTree {
    pub fn new(node: Operator) -> BTree {
        BTree {
            c1: None,
            c2: None,
            node,
        }
    }

    fn find_depth_value(&self, depth: usize) -> Vec<Option<Operator>> {
        let mut ret = Vec::new();
        if depth == 0 {
            return vec![Some(self.node.clone())]
        }
        match &self.c1 {
            Some(c1) => ret.append(&mut c1.find_depth_value(depth - 1)),
            None => ret.append(&mut vec![None; 2_usize.pow((depth - 1) as u32)])
        }
        match &self.c2 {
            Some(c2) => ret.append(&mut c2.find_depth_value(depth - 1)),
            None => ret.append(&mut vec![None; 2_usize.pow((depth - 1) as u32)])
        }
        ret

    }

    pub fn print(&self) {
        let mut staged_tree = Vec::new();
        let mut depth = 0;
        loop {
            let vec = self.find_depth_value(depth);
            if vec.iter().all(|x| x.is_none()) {
                break
            }
            staged_tree.push(vec);
            depth += 1;
        }
        let len = staged_tree.len();
        if len == 0 {
            return
        }
        let new_tree = staged_tree
            .iter()
            .map(|stage| stage
                .iter()
                .map(|ope| match ope {
                    Some(ope) => ope.to_string(),
                    None => String::from("_")
                })
                .collect::<Vec<String>>()
            ).collect::<Vec<Vec<String>>>();
        let max_ope_len = new_tree.iter().fold(0, |acc, stage| acc.max(stage.iter().fold(0, |acc, ope| acc.max(ope.len()))));
        for (depth, stage) in new_tree.iter().enumerate() {
            let mut ret = String::new();
            let mut padding = 0;
            for _ in 0..len - depth - 1 {
                padding = 2 * padding + 1;
            }
            for ope in stage {
                if depth != len - 1 {
                    for _ in 0..padding {
                        ret.push(' ');
                    }
                }
                ret = format!("{ret}{:>max_ope_len$} ", ope);
                if depth != len - 1 {
                    for _ in 0..padding {
                        ret.push(' ');
                    }
                }
            }
            println!("{ret}");
        }
    }

    pub fn insert_a(&mut self, sub_tree: BTree) {
        self.c1 = Some(Box::new(sub_tree));
    }

    pub fn insert_b(&mut self, sub_tree: BTree) {
        self.c2 = Some(Box::new(sub_tree));
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        if let Some(c1) = &self.c1 {
            ret = format!("{c1}");
        }
        if let Some(c2) = &self.c2 {
            ret = format!("{ret}{c2}");
        }
        format!("{ret} {}", self.node)
    }

    pub fn from_vec(formula: &mut Vec<Operator>) -> Result<BTree, String> {
        if let Some(last_op) = formula.pop() {
            let mut ret = match last_op {
                Operator::Var(_) | Operator::Number { .. } | Operator::Mat(_) => return Ok(BTree::new(last_op)),
                Operator::OpenParenthesis | Operator::CloseParenthesis | Operator::Equal => Err("unexpected operator {last_op} in btree")?,
                op => BTree::new(op)
            };
            ret.insert_b(BTree::from_vec(formula)?);
            ret.insert_a(BTree::from_vec(formula)?);
            return Ok(ret)
        }
        Err(String::from("Error while parsing formula"))
    }

    // pub fn eval(&self) -> bool {
    //     match calc_formula(&Box::new(self.clone())) {
    //         Ok(res) => res,
    //         Err(e) => {
    //             println!("{e}");
    //             false
    //         }
    //     }
    // }

    // pub fn find_nodes(&self, serch_fn: fn (&Operator) -> bool) -> Vec<Operator> {
    //     let mut ret: Vec<Operator> = Vec::new();

    //     if serch_fn(&self.node) {
    //         ret.push(self.node.clone());
    //     }
    //     if let Some(child_1) = &self.c1 {
    //         ret.append(&mut child_1.find_nodes(serch_fn));
    //     }
    //     if let Some(child_2) = &self.c2 {
    //         ret.append(&mut child_2.find_nodes(serch_fn));
    //     }

    //     ret
    // }
}

// fn calc_formula(tree: &Box<BTree>) -> Result<bool, String> {
//     match (&tree.node, &tree.c1, &tree.c2) {
//         (Operator::And, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? & calc_formula(&c2)?),
//         (Operator::Or, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? | calc_formula(&c2)?),
//         (Operator::Xor, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? ^ calc_formula(&c2)?),
//         (Operator::Material, Some(c1), Some(c2)) => Ok(!(calc_formula(&c1)? && !calc_formula(&c2)?)),
//         (Operator::Equal, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? == calc_formula(&c2)?),
//         (Operator::Not, Some(c1), None) => Ok(!calc_formula(&c1)?),
//         (Operator::Not, None, Some(c2)) => Ok(!calc_formula(&c2)?),
//         (Operator::B(b), None, None) => Ok(*b),
//         _ => {
//             return Err(String::from("Error while calculating formula"))
//         }
//     }
// }
