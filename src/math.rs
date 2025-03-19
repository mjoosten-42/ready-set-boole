fn adder(a: u32, b: u32) -> u32 {
    let mut carry = a & b;
    let mut res = a ^ b;

    while carry != 0 {
        let c = carry << 1;

        carry = res & c;
        res ^= c;
    }

    res
}

fn multiplier(a: u32, b: u32) -> u32 {
    let mut res = 0;

    for i in 0..u32::BITS {
        if b & (1 << i) != 0 {
            res = adder(res, a << i);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn zero() {
        let range = 0..3;

        for a in range.clone() {
            for b in range.clone() {
                println!("{a}, {b}");
                assert!(a + b == adder(a, b));
                assert!(a * b == multiplier(a, b));
            }
        }
    }

    #[test]
    fn rand() {
        let mut rng = rand::rng();

        for _ in 0..100000 {
            let a: u32 = rng.random();
            let b: u32 = rng.random();

            println!("{a}, {b}");
            assert!(a.wrapping_add(b) == adder(a, b));
            assert!(a.wrapping_mul(b) == multiplier(a, b));
        }
    }
}
