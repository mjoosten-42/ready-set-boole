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
}

#[cfg(test)]
mod tests {
    use rand::seq::IndexedRandom;

    use crate::Tree;

    #[test]
    fn rand() {
        let formula = formula(10);

        let mut tree: Tree = formula.parse().unwrap();

        tree.print();
        tree.to_nnf();
        tree.print();

        assert!(tree.is_nnf());
    }

    fn formula(len: usize) -> String {
        let operands: Vec<char> = ('A'..='F').collect();
        let unary_operators: Vec<char> = "!".chars().collect();
        let binary_operators: Vec<char> = "&|^>=".chars().collect();
        let operators: Vec<char> = "!&|^>=".chars().collect();

        let mut one: Vec<char> = operands.clone();
        one.extend(unary_operators.clone());

        let mut two: Vec<char> = operands.clone();
        two.extend(unary_operators.clone());
        two.extend(binary_operators.clone());

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
            
            score += match c { 'A'..='Z' => 1, '!' => 0, _ => -1 };

            formula.push(c);
        }

        while score > 1 {
            let c = *operators.choose(&mut rng).unwrap();
            
            score += match c { '!' => 0, _ => -1 };

            formula.push(c);
        }

        println!("score: {score}");
        println!("{formula}");
        
        formula
    }
}
