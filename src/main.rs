pub mod math;
pub mod rpn;

use rpn::*;

fn main() {
    let formula = "1011||=";
    let root = Node::parse(formula);
    
    root.print();
    println!("{:?}", eval_formula("1011||="));
}

