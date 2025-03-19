mod math;

use std::fmt::Binary;

use crate::math::multiplier;

fn main() {
    multiplier(1, 1);
}

pub fn bin(s: &str, n: impl Binary) {
    println!("{s:<6}: {n:032b}");
}
