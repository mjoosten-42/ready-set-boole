use rand::prelude::*;

// Z-order curve
pub fn map(x: u16, y: u16) -> f64 {
    let mut z: u32 = 0;

    for i in 0..u16::BITS {
        let a = (x & (1 << i)) as u32;
        let b = (y & (1 << i)) as u32;

        z |= a << i | b << (i + 1);
    }

    eprintln!("{z}");

    z as f64 / u32::MAX as f64
}

pub fn reverse_map(n: f64) -> (u16, u16) {
    let z = (n * u32::MAX as f64) as u32;
    let mut x = 0;
    let mut y = 0;

    eprintln!("{n}");

    for i in 0..u16::BITS {
        let a = z & (1 << i << i);
        let b = z & (1 << i << i << 1);

        x |= (a >> i) as u16;
        y |= (b >> i >> 1) as u16;
    }

    eprintln!("{x} {y}");

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: usize = 100;

    #[test]
    fn min() {
        assert_eq!(map(0, 0), 0.0);
        assert_eq!(reverse_map(0.0), (0, 0));
    }

    #[test]
    fn max() {
        assert_eq!(map(u16::MAX, u16::MAX), 1.0);
        assert_eq!(reverse_map(1.0), (u16::MAX, u16::MAX));
    }

    #[test]
    fn rand() {
        let mut rng = rand::rng();

        for _ in 0..N {
            let f = rng.random_range(0.0..1.0);
            let (x, y) = reverse_map(f);

            assert_eq!(map(x, y), f);
        }
    }
}
