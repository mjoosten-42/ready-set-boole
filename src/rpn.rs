use std::iter::repeat;

pub fn eval_formula(formula: &str) -> bool {
    Node::parse(formula).evaluate()
}

pub fn print_truth_table(formula: &str) {

    
}

#[derive(Clone, Debug)]
pub struct Node {
    symbol: char,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    fn new(symbol: char) -> Self {
        Self { symbol, left: None, right: None }
    }

    fn parse(formula: &str) -> Node {
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

    fn formula(&self) -> String {
        let mut formula = String::new();

        for node in [&self.left, &self.right] {
            if let Some(node) = node {
                formula.push_str(&node.formula());
            }
        }

        formula.push(self.symbol);

        formula
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
            _ => unreachable!(),
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
