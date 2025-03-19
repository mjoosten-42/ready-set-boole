mod math;
mod gray;
mod rpn;

use std::fmt::Binary;

fn main() { }

pub fn bin(s: &str, n: impl Binary) {
    println!("{s:<6}: {n:032b}");
}
