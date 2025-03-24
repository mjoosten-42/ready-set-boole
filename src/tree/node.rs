pub mod rewrite;

#[derive(Clone, Debug)]
pub struct Node {
    symbol: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(symbol: char, left: Option<Node>, right: Option<Node>) -> Self {
        Self {
            symbol,
            left: left.map(|node| Box::new(node)),
            right: right.map(|node| Box::new(node)),
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

    pub fn left(&self) -> Option<&Node> {
        self.left.as_deref()
    }

    pub fn right(&self) -> Option<&Node> {
        self.right.as_deref()
    }

    pub fn take_left(&mut self) -> Box<Node> {
        self.left.take().unwrap()
    }

    pub fn children(&self) -> impl Iterator<Item = &Node> {
        self.left.as_deref().into_iter().chain(self.right.as_deref().into_iter())
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
                
        let left = self.left().unwrap().evaluate(f);

        match self.symbol {
            '!' => return !left,
            _ => (),
        }

        let right = self.right().unwrap().evaluate(f);

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

