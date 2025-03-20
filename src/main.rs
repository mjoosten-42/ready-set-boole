use rpn::*;

pub mod math;
pub mod rpn;

fn main() {
    println!("{}", negation_normal_form("A!!0|"));
}
