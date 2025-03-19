fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let results = [0, 1, 3, 2, 6, 7, 5, 4, 12];

        for i in 0..9 {
            println!("{i}, {}", gray_code(i));
            assert!(gray_code(i) == results[i as usize]);
        }
    }
}
