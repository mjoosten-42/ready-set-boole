use std::fmt::Binary;
use rand::prelude::*;

pub fn adder(a: u32, b: u32) -> u32 {
    let mut carry = a & b;
    let mut res = a ^ b;

    while carry != 0 {
        let c = carry << 1;

        carry = res & c;
        res ^= c;
    }

    res
}

fn bin(s: &str, n: impl Binary) {
    println!("{s:<6}: {n:032b}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut rng = rand::rng();

        for _ in 0..100000 {
            let a: u32 = rng.random();
            let b = rng.random();
            let c = a.wrapping_add(b);

            bin("a", a);
            bin("b", b);
            println!("---------------- +");
            bin("c", c);
            println!();

            assert!(c == adder(a, b));
        }
    }
}
