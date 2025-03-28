#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Clause {
    // Operands
    Value(bool),
    Variable(char),

    // Operator
    Negation,
    Conjunction,
    Disjunction,
    Exclusive,
    Material,
    Equivalence,
}

impl Clause {
    pub fn from(c: char) -> Self {
        match c {
            '0' => Self::Value(false),
            '1' => Self::Value(true),
            'A'..='Z' => Self::Variable(c),
            '!' => Self::Negation,
            '&' => Self::Conjunction,
            '|' => Self::Disjunction,
            '^' => Self::Exclusive,
            '>' => Self::Material,
            '=' => Self::Equivalence,
            _ => panic!("Invalid clause"),
        }
    }
    
    pub fn to(&self) -> char {
        match *self {
            Self::Value(false) => '0',
            Self::Value(true) => '1',
            Self::Variable(v) => v,
            Self::Negation => '!',
            Self::Conjunction => '&',
            Self::Disjunction => '|',
            Self::Exclusive => '^',
            Self::Material => '>',
            Self::Equivalence => '=',
        }
    }
    
    pub fn symbol(&self) -> char {
        match self {
            Self::Value(false) => '⊥',
            Self::Value(true) => '⊤',
            Self::Negation => '¬',
            Self::Conjunction => '∧',
            Self::Disjunction => '∨',
            Self::Exclusive => '⊕',
            Self::Material => '⇒',
            Self::Equivalence => '⇔',
            _ => self.to(),
        }
    }
}

