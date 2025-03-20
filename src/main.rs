pub mod math;
pub mod rpn;

use rpn::*;

fn main() {
    println!("{:?}", eval_formula("1011||="));
}

