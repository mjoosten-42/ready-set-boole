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
        self.to_nnf();

        while !self.is_cnf() {
            self.foreach_mut(Self::distributivity);
            self.foreach_mut(Self::de_morgan);
            self.foreach_mut(Self::double_negation);
        }
    }
    
    pub fn is_nnf(&self) -> bool {
        match self.symbol {
            'A'..='Z' => true,
            '!' => {
                match self.left().symbol {
                    'A'..='Z' => true,
                    _ => false,
                }
            }
            '&' | '|' => self.left().is_nnf() && self.right().is_nnf(),
            _ => false,
        }
    }

    pub fn is_cnf(&self) -> bool {
        match self.symbol {
            'A'..='Z' => true,
            '!' => {
                match self.left().symbol {
                    'A'..='Z' => true,
                    _ => false,
                }
            }
            '&' => self.left().is_cnf() && self.right().is_cnf(),
            '|' => {
                self.left().symbol != '&' && self.right().symbol != '&' && self.left().is_cnf() && self.right().is_cnf()
            }
            _ => false,
        }
    }
    
    // Remove ⇔, ⇒ and ⊕
    pub fn simplify(&mut self) {
        self.foreach_mut(Self::equivalence);
        self.foreach_mut(Self::material_conditions);
        self.foreach_mut(Self::exclusivity);
    }

    // (A ⇔ B) ⇔ ((A ⇒ B) ∧ (B ⇒ A))
    fn equivalence(&mut self) {
        if self.symbol == '=' {
            self.symbol = '&';

            let left = self.left.take().unwrap();
            let right = self.right.take().unwrap();

            self.left = Some(Box::new(Node::new('>', Some(left.clone()), Some(right.clone()))));
            self.right = Some(Box::new(Node::new('>', Some(right), Some(left))));
        }
    }

    // (A ⇒ B) ⇔ (¬A ∨ B)
    pub fn material_conditions(&mut self) {
        if self.symbol == '>' {
            self.symbol = '|';

            let left = self.left.take().unwrap();

            self.left = Some(Box::new(Node::new('!', Some(left), None)));
        }
    }

    // A ⊕ B ⇔ (A ∨ B) ∧ ¬(A ∧ B)
    fn exclusivity(&mut self) {
        if self.symbol == '^' {
            self.symbol = '&';

            let left = self.left.take().unwrap();
            let right = self.right.take().unwrap();

            self.left = Some(Box::new(Node::new('|', Some(left.clone()), Some(right.clone()))));

            let expr = Box::new(Node::new('&', Some(left), Some(right)));

            self.right = Some(Box::new(Node::new('!', Some(expr), None)));
        }
    }

    // ¬(A ∨ B) ⇔ (¬A ∧ ¬B)
    // ¬(A ∧ B) ⇔ (¬A ∨ ¬B)
    fn de_morgan(&mut self) {
        if self.symbol == '!' {
            let left = self.left.as_mut().unwrap();
         
            if left.symbol == '|' || left.symbol == '&' {
                let right: Box<Node> = left.right.take().unwrap();

                self.symbol = match left.symbol {
                    '|' => '&',
                    '&' => '|',
                    _ => unreachable!(),
                };

                self.right = Some(Box::new(Node::new('!', Some(right), None)));
                left.symbol = '!';
            }
        }
    }

    // (¬¬A) ⇔ A
    fn double_negation(&mut self) {
        while self.symbol == '!' && self.left().symbol == '!' {
            let mut last = self.left.take().unwrap().left.take().unwrap();

            mem::swap(self, &mut last);
        }

    }

    // (A ∨ (B ∧ C)) ⇔ ((A ∨ B) ∧ (A ∨ C))
    fn distributivity(&mut self) {
        if self.symbol == '|' && (self.left().symbol == '&' || self.right().symbol == '&') {
            self.symbol = '&';

            let mut and = match self.left().symbol { '&' => self.left.take(), _ => self.right.take() }.unwrap();
            let other = match self.left { None => self.right.take(), _ => self.left.take() }.unwrap();
            let expr = Box::new(Node::new('|', Some(other.clone()), Some(and.left.take().unwrap())));

            and.symbol = '|';
            and.left = Some(other);

            self.left = Some(expr);
            self.right = Some(and);
        }
    }
    
    // Move conjunctions to the end of the formula
    pub fn right_balance_conjunctions(&mut self) {
        while self.symbol == '&' && self.left().symbol == '&' {
            let mut left: Box<Node> = self.left.take().unwrap();
            let right = left.right.take().unwrap();

            mem::swap(self, &mut left);

            left.left = Some(right);
            self.right = Some(left);
        }

        if self.symbol == '&' {
            self.right_mut().right_balance_conjunctions();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{formula, Tree};

    #[test]
    fn nnf() {
        let formula = formula(10);

        let mut tree: Tree = formula.parse().unwrap();

        tree.print();
        tree.to_nnf();
        tree.print();

        assert!(tree.is_nnf());
    }

    #[test]
    fn cnf() {
        let formula = formula(8);

        let mut tree: Tree = formula.parse().unwrap();

        tree.print();
        tree.to_cnf();
        tree.print();

        assert!(tree.is_cnf());

        let formula = tree.formula();

        if let Some(index) = formula.chars().position(|c| c == '&') {
            assert!(formula.chars().skip(index).all(|c| c == '&'));
        }
    }

}
