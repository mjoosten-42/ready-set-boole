use std::{fmt::Display, iter::repeat, ops::Deref, str::FromStr};

use super::{node::Node, Tree};

impl Tree {
    pub fn print(&self) {
        let mut nodes: Vec<Option<&Node>> = vec![Some(self.root.deref())];
        let mut depth = std::cmp::min(10, self.root.depth());

        // do while depth > 0
        loop {
            let spaces = repeat(" ").take((1 << depth) - 1).collect::<String>();

            for node in nodes.iter() {
                print!("{spaces}{}{spaces} ", if let Some(node) = node { node.symbol() } else { ' '});
            }

            println!("\n");

            let mut new = Vec::new();

            for node in nodes {
                if let Some(node) = node {
                    match node.symbol() {
                        'A'..='Z' | '0' | '1' => {
                            new.push(None);
                            new.push(None);
                        }
                        '¬' => {
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

impl FromStr for Tree {
    type Err = ();

    fn from_str(formula: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();

        for c in formula.chars() {
            let node = match c {
                'A'..='Z' | '0' | '1' => Node::new(c, None, None),
                '!' => Node::new(c, Some(Box::new(stack.pop().ok_or(())?)), None),
                '&' | '|' | '^' | '>' | '=' => {
                    let right = Some(Box::new(stack.pop().ok_or(())?));
                    let left = Some(Box::new(stack.pop().ok_or(())?));

                    Node::new(c, left, right)
                }
                _ => return Err(()),
            };

            stack.push(node);
        }

        let root = Box::new(stack.pop().ok_or(())?);

        Ok(Tree { root })
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formula())
    }
}
