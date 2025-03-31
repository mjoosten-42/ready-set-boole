pub mod clause;
pub mod rewrite;

use clause::*;
use itertools::Itertools;
use std::iter::repeat;

#[derive(Clone, Debug)]
pub struct Node {
    clause: Clause,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(clause: Clause, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self {
            clause,
            left,
            right,
        }
    }

    pub fn clause(&self) -> Clause {
        self.clause
    }

    pub fn left(&self) -> &Node {
        self.left.as_deref().unwrap()
    }

    pub fn _left_mut(&mut self) -> &mut Node {
        self.left.as_mut().unwrap()
    }

    pub fn right(&self) -> &Node {
        self.right.as_deref().unwrap()
    }

    pub fn right_mut(&mut self) -> &mut Node {
        self.right.as_mut().unwrap()
    }

    pub fn children(&self) -> impl Iterator<Item = &Node> {
        self.left
            .as_deref()
            .into_iter()
            .chain(self.right.as_deref().into_iter())
    }

    pub fn children_mut(&mut self) -> impl Iterator<Item = &mut Node> {
        self.left
            .as_deref_mut()
            .into_iter()
            .chain(self.right.as_deref_mut().into_iter())
    }

    pub fn foreach_mut(&mut self, f: fn(&mut Self)) {
        f(self);

        for child in self.children_mut() {
            child.foreach_mut(f);
        }
    }

    pub fn depth(&self) -> usize {
        self.children()
            .map(|node| node.depth() + 1)
            .max()
            .unwrap_or(0)
    }

    pub fn formula(&self) -> String {
        let mut formula = String::new();

        for child in self.children() {
            formula.push_str(&child.formula());
        }

        formula.push(self.clause.to());

        formula
    }

    pub fn evaluate(&self, f: impl Fn(char) -> bool + Copy) -> bool {
        let left = || self.left().evaluate(f);
        let right = || self.right().evaluate(f);

        match self.clause {
            Clause::Variable(v) => return f(v),
            Clause::Value(b) => return b,
            Clause::Negation => return !left(),
            Clause::Conjunction => left() & right(),
            Clause::Disjunction => left() | right(),
            Clause::Exclusive => left() ^ right(),
            Clause::Material => !left() | right(),
            Clause::Equivalence => left() == right(),
        }
    }

    pub fn evaluate_sets(
        &self,
        encompassing: &Vec<i32>,
        f: impl Copy + Fn(char) -> Vec<i32>,
    ) -> Vec<i32> {
        match self.clause {
            Clause::Variable(v) => return f(v),
            Clause::Value(_) => panic!(),
            _ => (),
        }

        let left = self.left().evaluate_sets(encompassing, f);

        match self.clause {
            Clause::Negation => {
                return encompassing
                    .clone()
                    .into_iter()
                    .filter(|x| !left.contains(x))
                    .collect();
            }
            _ => (),
        }

        let right = self.right().evaluate_sets(encompassing, f);
        let clone = right.clone();

        match self.clause {
            Clause::Conjunction => left.into_iter().filter(|x| right.contains(x)).collect(),
            Clause::Disjunction => left.into_iter().chain(right.into_iter()).unique().collect(),
            Clause::Exclusive => left
                .clone()
                .into_iter()
                .filter(move |x| !clone.clone().contains(x))
                .chain(right.into_iter().filter(|x| !left.contains(x)))
                .collect(),
            Clause::Material => encompassing
                .clone()
                .into_iter()
                .filter(|x| right.contains(x) || !left.contains(x))
                .unique()
                .collect(),
            Clause::Equivalence => encompassing
                .clone()
                .into_iter()
                .filter(|x| left.contains(x) == right.contains(x))
                .unique()
                .collect(),
            _ => unreachable!(),
        }
    }

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
