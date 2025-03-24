mod node;
mod string;
mod table;

use node::Node;

pub fn eval_formula(formula: &str) -> bool {
    let tree: Tree = formula.parse().expect("Invalid formula");

    tree.evaluate()
}

pub fn print_truth_table(formula: &str) {
    let tree: Tree = formula.parse().expect("Invalid formula");

    println!("{}", tree.truth_table());
}

#[derive(Clone, Debug)]
pub struct Tree {
    root: Box<Node>,
}

impl Tree {
    pub fn evaluate(&self) -> bool {
        self.root.evaluate(|_| panic!("Unsolved variables"))
    }
    
    pub fn evaluate_with(&self, f: impl Fn(char) -> bool + Copy) -> bool {
        self.root.evaluate(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        compare("10&", false);
        compare("10|", true);
        compare("11>", true);
        compare("10=", false);
        compare("1011||=", true);
    }

    fn compare(formula: &str, result: bool) {
        println!("{formula}: {result}");

        assert_eq!(eval_formula(formula), result);
    }
}
