
use crate::operator::Operator;
use std::collections::HashMap;

lazy_static! {
    static ref VARIABLES: HashMap<String, Vec<Operator>> = HashMap::new();
} 