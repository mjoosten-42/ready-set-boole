use itertools::Itertools;
use std::collections::HashSet;

pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut power = vec![Vec::new(), set.clone()];

    for elem in set.clone() {
        for s in powerset(set.clone().into_iter().filter(|&e| e != elem).collect()) {
            power.push(s);
        }
    }

    power
        .into_iter()
        .unique()
        .sorted_by(|a, b| match Ord::cmp(&a.len(), &b.len()) {
            std::cmp::Ordering::Equal => a.cmp(b),
            o @ _ => o,
        })
        .collect()
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()   
}

pub struct Set {
    elements: HashSet<i32>,
}

impl Set {
    pub fn from(set: Vec<i32>) -> Self {
        Self {
            elements: HashSet::from_iter(set.into_iter()),
        }
    }

    pub fn to(self) -> Vec<i32> {
        self.elements.into_iter().collect()
    }
}
