use std::mem;

use super::*;

impl Node {
    pub fn to_nnf(&mut self) {
        self.simplify();

        // Scary
        while !self.is_nnf() {
            self.foreach_mut(Self::de_morgan);
            self.foreach_mut(Self::double_negation);
        }
    }

    pub fn to_cnf(&mut self) {
        self.simplify();

        while !self.is_cnf() {
            self.foreach_mut(Self::distributivity);
            self.foreach_mut(Self::de_morgan);
            self.foreach_mut(Self::double_negation);
        }
    }

    pub fn is_nnf(&self) -> bool {
        match self.clause {
            Clause::Value(_) | Clause::Variable(_) => true,
            Clause::Negation => self.left().clause.is_operand(),
            Clause::Conjunction | Clause::Disjunction => self.children().all(Node::is_nnf),
            _ => false,
        }
    }

    pub fn is_cnf(&self) -> bool {
        match self.clause {
            Clause::Value(_) | Clause::Variable(_) => true,
            Clause::Negation => self.left().clause.is_operand(),
            Clause::Conjunction => self.children().all(Node::is_cnf),
            Clause::Disjunction => self
                .children()
                .all(|node| node.clause != Clause::Conjunction && node.is_cnf()),
            _ => false,
        }
    }

    // Remove ⇔, ⇒ and ⊕
    pub fn simplify(&mut self) {
        self.foreach_mut(Self::equivalence);
        self.foreach_mut(Self::implies);
        self.foreach_mut(Self::exclusivity);
    }

    // (A ⇔ B) ⇔ ((A ⇒ B) ∧ (B ⇒ A))
    fn equivalence(&mut self) {
        if self.clause == Clause::Equivalence {
            let left = self.left.take().unwrap();
            let right = self.right.take().unwrap();

            self.clause = Clause::Conjunction;
            self.left = Some(Box::new(Node::new(
                Clause::Material,
                Some(left.clone()),
                Some(right.clone()),
            )));
            self.right = Some(Box::new(Node::new(
                Clause::Material,
                Some(right),
                Some(left),
            )));
        }
    }

    // (A ⇒ B) ⇔ (¬A ∨ B)
    pub fn implies(&mut self) {
        if self.clause == Clause::Material {
            let left = self.left.take().unwrap();

            self.clause = Clause::Disjunction;
            self.left = Some(Box::new(Node::new(Clause::Negation, Some(left), None)));
        }
    }

    // A ⊕ B ⇔ (A ∨ B) ∧ ¬(A ∧ B)
    fn exclusivity(&mut self) {
        if self.clause == Clause::Exclusive {
            self.clause = Clause::Conjunction;

            let left = self.left.take().unwrap();
            let right = self.right.take().unwrap();

            self.left = Some(Box::new(Node::new(
                Clause::Disjunction,
                Some(left.clone()),
                Some(right.clone()),
            )));

            let expr = Box::new(Node::new(Clause::Conjunction, Some(left), Some(right)));

            self.right = Some(Box::new(Node::new(Clause::Negation, Some(expr), None)));
        }
    }

    // ¬(A ∨ B) ⇔ (¬A ∧ ¬B)
    // ¬(A ∧ B) ⇔ (¬A ∨ ¬B)
    fn de_morgan(&mut self) {
        if self.clause == Clause::Negation {
            let left = self.left.as_mut().unwrap();

            if left.clause == Clause::Conjunction || left.clause == Clause::Disjunction {
                let right: Box<Node> = left.right.take().unwrap();

                self.right = Some(Box::new(Node::new(Clause::Negation, Some(right), None)));
                self.clause = match left.clause {
                    Clause::Conjunction => Clause::Disjunction,
                    Clause::Disjunction => Clause::Conjunction,
                    _ => unreachable!(),
                };

                left.clause = Clause::Negation;
            }
        }
    }

    // (¬¬A) ⇔ A
    fn double_negation(&mut self) {
        while self.clause == Clause::Negation && self.left().clause == Clause::Negation {
            let mut last = self.left.take().unwrap().left.take().unwrap();

            mem::swap(self, &mut last);
        }
    }

    // (A ∨ (B ∧ C)) ⇔ ((A ∨ B) ∧ (A ∨ C))
    fn distributivity(&mut self) {
        if self.clause == Clause::Disjunction
            && self
                .children()
                .any(|node| node.clause == Clause::Conjunction)
        {
            self.clause = Clause::Conjunction;

            let mut and = match self.left().clause {
                Clause::Conjunction => self.left.take(),
                _ => self.right.take(),
            }
            .unwrap();
            let other = match self.left {
                None => self.right.take(),
                _ => self.left.take(),
            }
            .unwrap();
            let expr = Box::new(Node::new(
                Clause::Disjunction,
                Some(other.clone()),
                Some(and.left.take().unwrap()),
            ));

            and.clause = Clause::Disjunction;
            and.left = Some(other);

            self.left = Some(expr);
            self.right = Some(and);
        }
    }

    // Move conjunctions to the end of the formula
    pub fn right_balance_conjunctions(&mut self) {
        while self.clause == Clause::Conjunction && self.left().clause == Clause::Conjunction {
            let mut left: Box<Node> = self.left.take().unwrap();
            let right = left.right.take().unwrap();

            mem::swap(self, &mut left);

            left.left = Some(right);
            self.right = Some(left);
        }

        if self.clause == Clause::Conjunction {
            self.right_mut().right_balance_conjunctions();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{formula, tree::*};

    const N: usize = 10;
    const SIZE: usize = 3;

    #[test]
    fn rand() {
        for _ in 0..N {
            let formula = formula(SIZE);

            nnf(&formula);
            cnf(&formula);
        }
    }

    fn nnf(formula: &str) {
        let mut tree: Tree = formula.parse().unwrap();

        tree.print();
        tree.to_nnf();
        tree.print();

        assert!(tree.is_nnf());
    }

    fn cnf(formula: &str) {
        let mut tree: Tree = formula.parse().unwrap();

        tree.print();
        tree.to_cnf();
        tree.push_conjunctions();
        tree.print();

        let formula = tree.formula();

        if let Some(index) = formula.chars().position(|c| c == '&') {
            assert!(formula.chars().skip(index).all(|c| c == '&'));
        }
    }
}
