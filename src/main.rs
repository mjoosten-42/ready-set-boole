use tree::*;

pub mod math;
pub mod tree;

fn main() {
    let formula = "AB>";
    let mut tree: Tree = formula.parse().unwrap();

    tree.print();

    println!("{}", negation_normal_form(formula));
    
    tree.to_nnf();
    tree.print();
}
