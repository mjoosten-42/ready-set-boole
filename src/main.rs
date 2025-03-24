use rpn::*;

pub mod math;
pub mod rpn;

fn main() {
    let formula = "1011||=";
    let root = Node::parse(formula);

    println!("{}", negation_normal_form("A!!0|"));
    
    root.print();
    println!("{:?}", eval_formula("1011||="));
}
