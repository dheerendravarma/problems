/*
Test Cases
4
1 2 3
2 N 1 3 N
1 2 N 3 N 4 N
1 2 3 N N 4 6 N 5 N N 7 N

Output
2
3
4
3
*/

use std::io::{self, BufRead};
use tree::{build_tree, NodeRef, TreeLink};

// Returns the 1-indexed level number with the maximum sum of node values.
fn get_max_level_sum(root: &TreeLink) -> usize {
    if root.is_none() {
        return 0;
    }

    let mut current_level: Vec<NodeRef> = vec![root.clone().unwrap()];
    let mut max_sum = i64::MIN;
    let mut max_level = 1usize;
    let mut level = 1usize;

    while !current_level.is_empty() {
        let sum: i64 = current_level.iter().map(|n| n.borrow().data).sum();
        if sum > max_sum {
            max_sum = sum;
            max_level = level;
        }

        let mut next_level: Vec<NodeRef> = Vec::new();
        for node in &current_level {
            if let Some(l) = node.borrow().left.clone() {
                next_level.push(l);
            }
            if let Some(r) = node.borrow().right.clone() {
                next_level.push(r);
            }
        }
        current_level = next_level;
        level += 1;
    }
    max_level
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut results = Vec::new();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        results.push(get_max_level_sum(&build_tree(line.trim())));
    }
    for res in &results {
        println!("{}", res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree::build_tree;

    #[test]
    fn test_max_level_sum() {
        assert_eq!(get_max_level_sum(&build_tree("1 2 3")), 2);
        assert_eq!(get_max_level_sum(&build_tree("2 N 1 3 N")), 3);
        assert_eq!(get_max_level_sum(&build_tree("1 2 N 3 N 4 N")), 4);
        assert_eq!(
            get_max_level_sum(&build_tree("1 2 3 N N 4 6 N 5 N N 7 N")),
            3
        );
    }
}
