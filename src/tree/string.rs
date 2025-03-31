use std::{fmt::Display, str::FromStr};

use super::{
    Tree,
    node::{Node, clause::Clause},
};

impl FromStr for Tree {
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

        let root = Box::new(stack.pop().ok_or(())?);

        Ok(Tree { root })
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formula())
    }
}
