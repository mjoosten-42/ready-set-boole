use itertools::Itertools;
use std::iter::{once, repeat};

pub fn eval_formula(formula: &str) -> bool {
    Node::parse(formula).evaluate()
}

pub fn print_truth_table(formula: &str) {
    let variables: String = formula
        .chars()
        .filter(char::is_ascii_uppercase)
        .unique()
        .rev()
        .collect();

    let columns = "|".repeat(variables.len() + 2);

    println!(
        "{}",
        Itertools::intersperse(
            columns
                .chars()
                .interleave(variables.chars().rev().chain(once('='))),
            ' '
        )
        .collect::<String>()
    );

    println!(
        "{}",
        Itertools::intersperse(Itertools::intersperse(columns.chars(), '-'), '-')
            .collect::<String>()
    );

    for i in 0..(1 << variables.len()) {
        let formula: String = formula.chars().map(|c| vtob(c, i, &variables)).collect();

        println!(
            "{}",
            variables
                .chars()
                .rev()
                .chain(once(match eval_formula(&formula) {
                    false => '0',
                    true => '1',
                }))
                .fold(String::from("|"), |acc, c| format!(
                    "{acc} {} |",
                    vtob(c, i, &variables)
                ))
        );
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    let mut root = Node::parse(formula);

    while !root.is_nnf() {
        root.print();
        root.elim_dbl_neg();

        break;
    }
    
    root.print();

    root.formula()
}

fn vtob(c: char, n: u32, variables: &str) -> char {
    match c {
        'A'..'Z' => {
            let pos = variables.chars().position(|d| d == c).unwrap();

            match n & 1 << pos {
                0 => '0',
                _ => '1',
            }
        }
        _ => c,
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    symbol: char,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    fn new(symbol: char) -> Self {
        Self {
            symbol,
            left: None,
            right: None,
        }
    }

    pub fn parse(formula: &str) -> Node {
        let mut stack = Vec::new();

        for c in formula.chars() {
            let mut node = Node::new(c);

            match c {
                'A'..'Z' => stack.push(node),
                '0' | '1' => stack.push(node),
                '!' => {
                    node.left = Some(Box::new(stack.pop().expect("Insufficient operands")));

                    stack.push(node);
                }
                '&' | '|' | '^' | '>' | '=' => {
                    node.right = Some(Box::new(stack.pop().expect("Insufficient operands")));
                    node.left = Some(Box::new(stack.pop().expect("Insufficient operands")));

                    stack.push(node);
                }
                _ => panic!("Invalid symbol: {c}"),
            }
        }

        let root = stack.pop().unwrap();

        if !stack.is_empty() {
            panic!("Too many operands");
        }

        root
    }

    pub fn formula(&self) -> String {
        let mut formula = String::new();

        for node in [&self.left, &self.right] {
            if let Some(node) = node {
                formula.push_str(&node.formula());
            }
        }

        formula.push(self.symbol);

        formula
    }

    fn is_nnf(&self) -> bool {
        let mut nnf = match self.symbol { '&' | '|' => true, _ => false };

        for node in [&self.left, &self.right] {
            if let Some(node) = node {
                nnf &= node.is_nnf();
            }
        }

        nnf
    }

    fn evaluate(&self) -> bool {
        match self.symbol {
            'A'..'Z' => panic!("Unsolved variables"),
            '0' => false,
            '1' => true,
            _ => {
                let left = self.left.as_ref().unwrap().evaluate();

                match self.symbol {
                    '!' => !left,
                    _ => {
                        let right = self.right.as_ref().unwrap().evaluate();

                        match self.symbol {
                            '&' => left & right,
                            '|' => left | right,
                            '^' => left ^ right,
                            '>' => !left | right,
                            '=' => left == right,
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
    }

    fn symbol(&self) -> char {
        match self.symbol {
            '0' => '⊥',
            '1' => '⊤',
            '!' => '¬',
            '&' => '∧',
            '|' => '∨',
            '^' => '⊕',
            '>' => '⇒',
            '=' => '⇔',
            c @ _ => c,
        }
    }

    fn elim_dbl_neg(&mut self) {
        eprintln!("self: {}", self.symbol());

        if self.symbol == '!' {
            let left = self.left.as_mut().unwrap();

            if left.symbol == '!' {
                let n: Box<Node> = left.left.clone().unwrap();

                self.left = Some(n);
            }
        }
        
        for node in [&mut self.left, &mut self.right] {
            if let Some(node) = node {
                node.elim_dbl_neg();
            }
        }
    }

    fn depth(&self) -> usize {
        let mut depth = 0;

        if let Some(left) = &self.left {
            depth = 1 + left.depth();
        }

        if let Some(right) = &self.right {
            depth = std::cmp::max(depth, 1 + right.depth());
        }

        depth
    }

    pub fn print(&self) {
        let mut nodes = vec![Some(self.clone())];
        let mut depth = self.depth() + 1;

        while depth > 0 {
            depth -= 1;
            let spaces = (1 << depth) - 1;

            for node in nodes.iter() {
                print!("{}", repeat(" ").take(spaces).collect::<String>());

                if let Some(node) = node {
                    print!("{}", node.symbol());
                } else {
                    print!("-");
                }

                print!("{}", repeat(" ").take(spaces).collect::<String>());
                print!(" ");
            }

            println!("");

            let mut new = Vec::new();

            for node in nodes {
                if let Some(node) = node {
                    new.push(node.left.clone().and_then(|b: Box<Node>| Some(*b)));
                    new.push(node.right.clone().and_then(|b: Box<Node>| Some(*b)));
                } else {
                    new.push(None);
                    new.push(None);
                }
            }
            println!("");

            nodes = new;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        compare("10&", false);
        compare("10|", true);
        compare("11>", true);
        compare("10=", false);
        compare("1011||=", true);
    }

    fn compare(formula: &str, result: bool) {
        println!("{formula}: {result}");

        assert_eq!(eval_formula(formula), result);
        assert_eq!(Node::parse(formula).formula(), formula);
    }
}
