use rand::seq::IndexedRandom;

pub mod curve;
pub mod math;
pub mod set;
pub mod tree;

fn main() {}

pub fn formula(len: usize) -> String {
    let operands: Vec<char> = ('A'..='F').collect();
    let unary_operators: Vec<char> = "!".chars().collect();
    let operators: Vec<char> = "!&|^>=".chars().collect();

    let mut one: Vec<char> = operands.clone();
    one.extend(unary_operators.clone());

    let mut two: Vec<char> = operands.clone();
    two.extend(operators.clone());

    let mut formula = String::new();
    let mut rng = rand::rng();
    let mut score = 0;

    for _ in 0..len {
        let source = match score {
            0 => &operands,
            1 => &one,
            _ => &two,
        };

        let c = *source.choose(&mut rng).unwrap();

        score += match c {
            'A'..='Z' => 1,
            '!' => 0,
            _ => -1,
        };

        formula.push(c);
    }

    while score > 1 {
        let c = *operators.choose(&mut rng).unwrap();

        score += match c {
            '!' => 0,
            _ => -1,
        };

        formula.push(c);
    }

    formula
}
