use super::*;

impl Node {
    pub fn to_nnf(parent: &mut Box<Self>) {
        Self::material_conditions(parent);
        Self::double_negation(parent);
    }

    pub fn material_conditions(parent: &mut Box<Self>) {
        let node = &mut *parent;

        if node.symbol == '>' {
            node.symbol = '|';

            let left = *node.take_left();

            node.left = Some(Box::new(Node::new('!', Some(left), None)));
        }

        for child in [&mut node.left, &mut node.right] {
            if let Some(child) = child {
                Self::to_nnf(child);
            }
        }
    }

    pub fn double_negation(parent: &mut Box<Self>) {
        let node: &mut Node  = &mut *parent;

        if node.symbol == '!' {
            let left = node.left().unwrap();

            if left.symbol == '!' {
                let mut left: Node = *node.take_left();
                let child: Box<Node> = left.take_left();

                *parent = child;

                return Self::to_nnf(parent);
            }
        }

        let node = &mut *parent;

        for child in [&mut node.left, &mut node.right] {
            if let Some(child) = child {
                Self::to_nnf(child);
            }
        }
    }
}
