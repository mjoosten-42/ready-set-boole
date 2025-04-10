pub mod clause;
pub mod rewrite;
pub mod string;
pub mod table;

use clause::*;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Node {
    clause: Clause,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(clause: Clause, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self {
            clause,
            left,
            right,
        }
    }

    fn clause(&self) -> Clause {
        self.clause
    }

    fn left(&self) -> &Node {
        self.left.as_deref().unwrap()
    }

    fn right(&self) -> &Node {
        self.right.as_deref().unwrap()
    }

    fn right_mut(&mut self) -> &mut Node {
        self.right.as_mut().unwrap()
    }

    fn children(&self) -> impl Iterator<Item = &Node> {
        self.left
            .as_deref()
            .into_iter()
            .chain(self.right.as_deref().into_iter())
    }

    fn children_mut(&mut self) -> impl Iterator<Item = &mut Node> {
        self.left
            .as_deref_mut()
            .into_iter()
            .chain(self.right.as_deref_mut().into_iter())
    }

    fn foreach_mut(&mut self, f: fn(&mut Self)) {
        f(self);

        for child in self.children_mut() {
            child.foreach_mut(f);
        }
    }

    fn depth(&self) -> usize {
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

    // Assume only absolute values as operands
    pub fn evaluate(&self) -> bool {
        self.evaluate_with(|_| panic!("Unsolved variables"))
    }

    fn evaluate_with(&self, f: impl Fn(char) -> bool + Copy) -> bool {
        let left = || self.left().evaluate_with(f);
        let right = || self.right().evaluate_with(f);

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
}
