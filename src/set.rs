use itertools::Itertools;
use crate::tree::*;

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
    let tree: Tree = formula.parse().unwrap();
    let encompassing: Vec<i32> = sets.clone().into_iter().flatten().unique().collect();
    let variables: String = formula.chars().filter(char::is_ascii_uppercase).collect();

    tree.evaluate_sets(&encompassing, |c| sets[variables.chars().position(|d| d == c).unwrap()].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        compare("AB&", vec!(vec!(0, 1, 2), vec!(0, 3, 4)), vec!(0));
        compare("AB|", vec!(vec!(0, 1, 2), vec!(3, 4, 5)), vec!(0, 1, 2, 3, 4, 5));
        compare("A!", vec!(vec!(0, 1, 2)), vec!());
    }

    fn compare(formula: &str, sets: Vec<Vec<i32>>, res: Vec<i32>) {
        assert_eq!(res, eval_set(formula, sets));
    }
}
