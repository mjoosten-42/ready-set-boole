use node::Node;

pub mod curve;
pub mod math;
pub mod node;
pub mod set;

fn main() {
    let formula = "AB=C=";
    let mut tree: Node = formula.parse().unwrap();

    tree.to_cnf();
    tree.unbalance();
    tree.print();
}

pub fn eval_formula(formula: &str) -> bool {
    formula.parse::<Node>().unwrap().evaluate()
}

pub fn print_truth_table(formula: &str) {
    println!("{}", formula.parse::<Node>().unwrap().truth_table());
}

pub fn negation_normal_form(formula: &str) -> String {
    let mut tree: Node = formula.parse().unwrap();

    tree.to_nnf();
    tree.formula()
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let mut tree: Node = formula.parse().unwrap();

    tree.to_cnf();
    tree.unbalance();
    tree.formula()
}

pub fn sat(formula: &str) -> bool {
    formula.parse::<Node>().unwrap().sat()
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
        compare_nnf("AB&!", "A!B!|");
        compare_nnf("AB|!", "A!B!&");
        compare_nnf("AB>", "A!B|");
        // compare_nnf("AB=", "AB&A!B!&|"); // Uses a different rewrite rule
        compare_nnf("AB|C&!", "A!B!&C!|");
    }

    #[test]
    fn cnf() {
        compare_cnf("AB&!", "A!B!|");
        compare_cnf("AB|!", "A!B!&");
        compare_cnf("AB|C&", "AB|C&");
        compare_cnf("AB|C|D|", "ABCD|||");
        compare_cnf("AB&C&D&", "ABCD&&&");
        compare_cnf("AB&!C!|", "A!B!C!||");
        compare_cnf("AB|!C!&", "A!B!C!&&");
    }

    #[test]
    fn sat() {
        assert!(super::sat("AB|"));
        assert!(super::sat("AB&"));
        assert!(!super::sat("AA!&"));
        assert!(!super::sat("AA^"));
    }

    fn compare(formula: &str, result: bool) {
        println!("{formula}: {result}");

        assert_eq!(eval_formula(formula), result);
    }

    fn compare_nnf(formula: &str, answer: &str) {
        let nnf = negation_normal_form(formula);

        println!("{formula}: {nnf} != {answer}");

        assert_eq!(nnf, answer);
    }

    fn compare_cnf(formula: &str, answer: &str) {
        let cnf = conjunctive_normal_form(formula);

        println!("{formula}: {cnf} - {answer}");

        assert_eq!(cnf, answer);
    }
}
