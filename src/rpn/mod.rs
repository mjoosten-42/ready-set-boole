enum Token {
    Operand(bool),
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator),
}

struct Operator {
    children: Vec<Token>,
}

enum OperatorKind {
    Negation,               // NOT
    Conjunction,            // AND
    Disjunction,            // OR
    ExclusiveDisjunction,   // XOR
    MaterialCondition,      // IPLY
    LogicalEquivalence,     // EQ
}

fn parse(formula: &str) -> Token {
    let mut stack = Vec::new();


    
}
