use std::mem;

// Z-order curve
pub fn map(x: u16, y: u16) -> f64 {
    let mut z: u64 = 0;

    // Interleave x and y
    for i in 0..u16::BITS {
        let a = (x & (1 << i)) as u64;
        let b = (y & (1 << i)) as u64;

        z |= a << i | b << (i + 1);
    }

    // f64's mantissa is 52 bits, plenty to fit our two interleaved u16's
    unsafe { mem::transmute(z) }
}

pub fn reverse_map(n: f64) -> (u16, u16) {
    let z: u64 = unsafe { mem::transmute(n) };
    let mut x = 0;
    let mut y = 0;

    for i in 0..u16::BITS {
        let a = z & (1 << i << i);
        let b = z & (1 << i << i << 1);

        x |= (a >> i) as u16;
        y |= (b >> i >> 1) as u16;
    }

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    const N: usize = 1000;

    #[test]
    fn rand() {
        let mut rng = rand::rng();

        for _ in 0..N {
            let x: u16 = rng.random();
            let y: u16 = rng.random();

            let f = map(x, y);

            assert!(f >= 0.0 && f <= 1.0);

            eprintln!("{f}");

            assert_eq!(reverse_map(f), (x, y));
        }
    }

    #[test]
    fn minimax() {
        assert_eq!(reverse_map(map(0, 0)), (0, 0));
        assert_eq!(reverse_map(map(u16::MAX, u16::MAX)), (u16::MAX, u16::MAX));
    }
}
