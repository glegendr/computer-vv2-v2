use crate::{operator::Operator, btree::BTree};

pub fn calc(input: &Vec<Operator>) -> Result<BTree, String> {
    let tree = BTree::from_vec(&input)?;
    tree.eval()
}