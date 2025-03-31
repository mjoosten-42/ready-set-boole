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

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut res = 0;

    for i in 0..u32::BITS {
        if b & (1 << i) != 0 {
            res = adder(res, a << i);
        }
    }

    res
}

pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    const N: usize = 10000;

    #[test]
    fn small() {
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

        for _ in 0..N {
            let a: u32 = rng.random();
            let b: u32 = rng.random();

            println!("{a}, {b}");
            assert!(a.wrapping_add(b) == adder(a, b));
            assert!(a.wrapping_mul(b) == multiplier(a, b));
        }
    }

    #[test]
    fn subject() {
        let results = [0, 1, 3, 2, 6, 7, 5, 4, 12];

        for i in 0..9 {
            println!("{i}, {}", gray_code(i));
            assert!(gray_code(i) == results[i as usize]);
        }
    }
}
