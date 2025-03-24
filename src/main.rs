use tree::*;

pub mod math;
pub mod tree;

fn main() {
    let formula = "AB&C|";
    let tree: Tree = formula.parse().unwrap();

    tree.print();
    print_truth_table(formula);
}
