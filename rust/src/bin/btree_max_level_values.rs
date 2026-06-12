/*
Test Cases
4
1 2 3
2 N 1 3 N
1 2 N 3 N 4 N
1 2 3 N N 4 6 N 5 N N 7 N

Output
1 3
2 1 3
1 2 3 4
1 3 6 5 7
*/

use std::io::{self, BufRead};
use tree::{build_tree, NodeRef, TreeLink};

fn get_max_level_values(root: &TreeLink) -> Vec<i64> {
    let mut result = Vec::new();
    if root.is_none() {
        return result;
    }

    let mut current_level: Vec<NodeRef> = vec![root.clone().unwrap()];

    while !current_level.is_empty() {
        let max_val = current_level.iter().map(|n| n.borrow().data).max().unwrap();
        result.push(max_val);

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
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut results = Vec::new();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        results.push(get_max_level_values(&build_tree(line.trim())));
    }
    for res in &results {
        let strs: Vec<String> = res.iter().map(|v| v.to_string()).collect();
        println!("{}", strs.join(" "));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree::build_tree;

    #[test]
    fn test_max_level_values() {
        assert_eq!(get_max_level_values(&build_tree("1 2 3")), vec![1, 3]);
        assert_eq!(
            get_max_level_values(&build_tree("2 N 1 3 N")),
            vec![2, 1, 3]
        );
        assert_eq!(
            get_max_level_values(&build_tree("1 2 N 3 N 4 N")),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            get_max_level_values(&build_tree("1 2 3 N N 4 6 N 5 N N 7 N")),
            vec![1, 3, 6, 5, 7]
        );
    }
}
