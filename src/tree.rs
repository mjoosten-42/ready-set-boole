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

pub fn negation_normal_form(formula: &str) -> String {
    let mut tree: Tree = formula.parse().expect("Invalid formula");

    tree.to_nnf();
    tree.formula()
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let mut tree: Tree = formula.parse().expect("Invalid formula");

    tree.to_cnf();
    tree.push_conjunctions();
    tree.formula()
}

pub fn sat(formula: &str) -> bool {
    let tree: Tree = formula.parse().expect("Invalid formula");

    tree.sat()
}

#[derive(Clone, Debug)]
pub struct Tree {
    root: Box<Node>,
}

impl Tree {
    pub fn formula(&self) -> String {
        self.root.formula()
    }

    pub fn evaluate(&self) -> bool {
        self.root.evaluate(|_| panic!("Unsolved variables"))
    }

    pub fn evaluate_with(&self, f: impl Fn(char) -> bool + Copy) -> bool {
        self.root.evaluate(f)
    }

    pub fn evaluate_sets(&self, all: &Vec<i32>, f: impl Copy + Fn(char) -> Vec<i32>) -> Vec<i32> {
        self.root.evaluate_sets(all, f)
    }

    pub fn to_nnf(&mut self) {
        self.root.to_nnf();
    }

    pub fn to_cnf(&mut self) {
        self.root.to_cnf();
    }

    pub fn push_conjunctions(&mut self) {
        self.root.right_balance_conjunctions();
    }

    pub fn is_nnf(&self) -> bool {
        self.root.is_nnf()
    }

    pub fn is_cnf(&self) -> bool {
        self.root.is_cnf()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval() {
        compare("10&", false);
        compare("10|", true);
        compare("11>", true);
        compare("10=", false);
        compare("1011||=", true);
    }

    #[test]
    fn nnf() {
        compare_nnf("AB|!", "A!B!&");
        compare_nnf("AB>", "A!B|");
        compare_nnf("AB|C&!", "A!B!&C!|");
    }

    fn compare_nnf(formula: &str, answer: &str) {
        let nnf = negation_normal_form(formula);

        println!("{formula}: {nnf} - {answer}");

        assert_eq!(nnf, answer);
    }

    fn compare(formula: &str, result: bool) {
        println!("{formula}: {result}");

        assert_eq!(eval_formula(formula), result);
    }
}
