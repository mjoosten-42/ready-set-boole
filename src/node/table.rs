use super::Node;
use itertools::Itertools;
use std::iter::once;

impl Node {
    pub fn truth_table(&self) -> String {
        let mut table = Vec::new();
        let variables: String = self.variables().chars().rev().collect();
        let columns = "|".repeat(variables.len() + 2);

        table.push(format!(
            "{}",
            Itertools::intersperse(
                columns
                    .chars()
                    .interleave(variables.chars().rev().chain(once('='))),
                ' '
            )
            .collect::<String>()
        ));

        table.push(format!(
            "{}",
            Itertools::intersperse(Itertools::intersperse(columns.chars(), '-'), '-')
                .collect::<String>()
        ));

        let end = 1 << variables.len();

        for i in 0..end {
            let mapping = |c| match i & 1 << variables.chars().position(|d| d == c).unwrap() {
                0 => false,
                _ => true,
            };
            let btoc = |b: bool| (b as u8 + '0' as u8) as char;

            table.push(format!(
                "{} {} |",
                variables
                    .chars()
                    .rev()
                    .fold(String::from("|"), |acc, c| format!(
                        "{acc} {} |",
                        btoc(mapping(c))
                    )),
                btoc(self.evaluate_with(mapping))
            ));
        }

        table.join("\n")
    }

    pub fn sat(&self) -> bool {
        let variables = self.variables();
        let end = 1 << variables.len();

        for i in 0..end {
            let mapping = |c| match i & 1 << variables.chars().position(|d| d == c).unwrap() {
                0 => false,
                _ => true,
            };

            if self.evaluate_with(mapping) {
                return true;
            }
        }

        false
    }

    pub fn variables(&self) -> String {
        self.formula()
            .chars()
            .filter(char::is_ascii_uppercase)
            .unique()
            .sorted()
            .collect()
    }
}
