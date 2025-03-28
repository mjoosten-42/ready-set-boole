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

pub struct Set {
    elements: HashSet<i32>,
}

impl Set {
    pub fn new(set: Vec<i32>) -> Self {
        Self {
            elements: HashSet::from_iter(set.into_iter()),
        }
    }
}
