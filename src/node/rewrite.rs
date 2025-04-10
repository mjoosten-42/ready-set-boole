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

    // Move con- and disjunctions to the end of the formula,
    // but keep conjunctions at the top
    pub fn unbalance(&mut self) {
        while self.children().count() == 2
            && self.left().children().count() == 2
            && self.clause >= self.left().clause
        {
            self.rotate_right();
        }

        if self.children().count() == 2 {
            self.right_mut().unbalance();
        }
    }

    fn rotate_right(&mut self) {
        let mut left: Box<Node> = self.left.take().unwrap();
        let right = left.right.take().unwrap();

        mem::swap(self, &mut left);

        left.left = Some(right);
        self.right = Some(left);
    }
}

#[cfg(test)]
mod tests {
    use rand::seq::IndexedRandom;

    use crate::node::*;

    const N: usize = 10;
    const SIZE: usize = 5;

    #[test]
    fn rand() {
        for _ in 0..N {
            let formula = formula(SIZE);

            nnf(&formula);
            cnf(&formula);
        }
    }

    fn nnf(formula: &str) {
        let mut tree: Node = formula.parse().unwrap();

        tree.print();
        tree.to_nnf();
        tree.print();

        assert!(tree.is_nnf());
    }

    fn cnf(formula: &str) {
        let mut tree: Node = formula.parse().unwrap();

        tree.print();
        tree.to_cnf();
        tree.unbalance();
        tree.print();

        assert!(tree.is_cnf());

        let formula = tree.formula();

        if let Some(index) = formula.chars().position(|c| c == '&') {
            assert!(formula.chars().skip(index).all(|c| c == '&'));
        }
    }

    fn formula(len: usize) -> String {
        let operands: Vec<char> = ('A'..='F').collect();
        let unary_operators: Vec<char> = "!".chars().collect();
        let operators: Vec<char> = "!&|^>=".chars().collect();

        let mut one: Vec<char> = operands.clone();
        one.extend(unary_operators.clone());

        let mut two: Vec<char> = operands.clone();
        two.extend(operators.clone());

        let mut formula = String::new();
        let mut rng = rand::rng();
        let mut score = 0;

        for _ in 0..len {
            let source = match score {
                0 => &operands,
                1 => &one,
                _ => &two,
            };

            let c = *source.choose(&mut rng).unwrap();

            score += match c {
                'A'..='Z' => 1,
                '!' => 0,
                _ => -1,
            };

            formula.push(c);
        }

        while score > 1 {
            let c = *operators.choose(&mut rng).unwrap();

            score += match c {
                '!' => 0,
                _ => -1,
            };

            formula.push(c);
        }

        formula
    }
}
