use tree::*;

pub mod math;
pub mod tree;

fn main() {
    let formula = "AB|!C!&";
    let mut tree: Tree = formula.parse().unwrap();
    let table = tree.truth_table();

    tree.print();
    tree.to_cnf();
    tree.print();

    assert_eq!(table, tree.truth_table());
    assert!(tree.is_nnf());
}
