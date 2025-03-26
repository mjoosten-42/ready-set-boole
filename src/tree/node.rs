pub mod rewrite;

#[derive(Clone, Debug)]
pub struct Node {
    symbol: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(symbol: char, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self {
            symbol,
            left,
            right,
        }
    }

    pub fn symbol(&self) -> char {
        match self.symbol {
            '0' => '⊥',
            '1' => '⊤',
            '!' => '¬',
            '&' => '∧',
            '|' => '∨',
            '^' => '⊕',
            '>' => '⇒',
            '=' => '⇔',
            c @ _ => c,
        }
    }

    pub fn left(&self) -> &Node {
        self.left.as_deref().unwrap()
    }

    pub fn left_mut(&mut self) -> &mut Node {
        self.left.as_mut().unwrap()
    }

    pub fn right(&self) -> &Node {
        self.right.as_deref().unwrap()
    }
    
    pub fn right_mut(&mut self) -> &mut Node {
        self.right.as_mut().unwrap()
    }


    pub fn children(&self) -> impl Iterator<Item = &Node> {
        self.left.as_deref().into_iter().chain(self.right.as_deref().into_iter())
    }
    
    pub fn children_mut(&mut self) -> impl Iterator<Item = &mut Node> {
        self.left.as_deref_mut().into_iter().chain(self.right.as_deref_mut().into_iter())
    }
    
    pub fn foreach_mut(&mut self, f: fn(&mut Self)) {
        f(self);

        for child in self.children_mut() {
            child.foreach_mut(f);
        }
    }

    pub fn depth(&self) -> usize {
        let mut depth = 0;

        for child in self.children() {
            depth = std::cmp::max(depth, 1 + child.depth());
        }

        depth
    }
    
    pub fn formula(&self) -> String {
        let mut formula = String::new();

        for child in self.children() {
            formula.push_str(&child.formula());
        }

        formula.push(self.symbol);

        formula
    }

    pub fn evaluate(&self, f: impl Fn(char) -> bool + Copy) -> bool {
        match self.symbol {
            c @ 'A'..='Z' => return f(c),
            '0' => return false,
            '1' => return true,
            _ => (),
        }
                
        let left = self.left().evaluate(f);

        match self.symbol {
            '!' => return !left,
            _ => (),
        }

        let right = self.right().evaluate(f);

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

