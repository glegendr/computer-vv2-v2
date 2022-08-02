use crate::{operator::Operator, calculation::calc};

pub fn assign(input: &Vec<Vec<Operator>>) {
    let first_part = input.get(0).unwrap();

    match first_part.len() {
        0 => println!("Empty input"),
        1 => variable_assignation(&input),
        2 => function_assignation(&input),
        _ => {
            println!("Assignation is only available for variable or function. Eq:");
            println!(">> x = 50 - 8");
            println!(">> f(x) = 3 * x + 2");
        }
    }
}

fn variable_assignation(input: &Vec<Vec<Operator>>) {
    let first_part = input.get(0).unwrap();

    match first_part.get(0).unwrap() {
        Operator::Var(value) => {
            println!("{value} = ");
            println!("{:?}", calc(input.get(1).unwrap()));
        },
        _ => {
            println!("Assignation is only available for variable or function. Eq:");
            println!(">> x = 50 - 8");
        }
    }
}

fn function_assignation(input: &Vec<Vec<Operator>>) {
    let first_part = input.get(0).unwrap();

    match (first_part.get(0).unwrap(), first_part.get(1).unwrap()) {
        (Operator::Var(name), Operator::Var(value)) => {
            println!("{name}({value}) = ");
        },
        _ => {
            println!("Assignation is only available for variable or function. Eq:");
            println!(">> f(x) = 3 * x + 2");
        }
    }
}