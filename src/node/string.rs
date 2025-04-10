use std::{fmt::Display, iter::repeat, str::FromStr};

use super::{Clause, Node};

impl Node {
    pub fn print(&self) {
        let mut nodes: Vec<Option<&Self>> = vec![Some(self)];
        let mut depth = std::cmp::min(10, self.depth());

        // do while depth > 0
        loop {
            let spaces = repeat(" ").take((1 << depth) - 1).collect::<String>();

            for node in nodes.iter() {
                print!(
                    "{spaces}{}{spaces} ",
                    if let Some(node) = node {
                        node.clause().symbol()
                    } else {
                        ' '
                    }
                );
            }

            println!("\n");

            let mut new = Vec::new();

            for node in nodes {
                if let Some(node) = node {
                    match node.clause() {
                        Clause::Value(_) | Clause::Variable(_) => {
                            new.push(None);
                            new.push(None);
                        }
                        Clause::Negation => {
                            new.push(Some(node.left()));
                            new.push(None);
                        }
                        _ => {
                            new.push(Some(node.left()));
                            new.push(Some(node.right()));
                        }
                    }
                } else {
                    new.push(None);
                    new.push(None);
                }
            }

            nodes = new;

            if depth == 0 {
                break;
            }

            depth -= 1;
        }
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(formula: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();

        for c in formula.chars() {
            let clause = Clause::from(c);

            let node = match c {
                'A'..='Z' | '0' | '1' => Node::new(clause, None, None),
                '!' => Node::new(clause, Some(Box::new(stack.pop().ok_or(())?)), None),
                '&' | '|' | '^' | '>' | '=' => {
                    let right = Some(Box::new(stack.pop().ok_or(())?));
                    let left = Some(Box::new(stack.pop().ok_or(())?));

                    Node::new(clause, left, right)
                }
                _ => return Err(()),
            };

            stack.push(node);
        }

        let root = stack.pop().ok_or(())?;

        Ok(root)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formula())
    }
}
