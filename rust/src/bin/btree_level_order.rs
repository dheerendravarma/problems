/*
Test Cases
4
1 2 3
2 N 1 3 N
1 2 N 3 N 4 N
1 2 3 N N 4 6 N 5 N N 7 N

Output
1 2 3
2 3 1
1 2 3 4
1 2 3 4 5 6 7
*/

use std::collections::VecDeque;
use std::io::{self, BufRead};
use tree::{build_tree, TreeLink};

fn level_order(root: &TreeLink) -> Vec<i64> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    if let Some(r) = root.clone() {
        queue.push_back(r);
    }
    while let Some(node) = queue.pop_front() {
        result.push(node.borrow().data);
        if let Some(left) = node.borrow().left.clone() {
            queue.push_back(left);
        }
        if let Some(right) = node.borrow().right.clone() {
            queue.push_back(right);
        }
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
        results.push(level_order(&build_tree(line.trim())));
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
    fn test_level_order() {
        assert_eq!(level_order(&build_tree("1 2 3")), vec![1, 2, 3]);
        assert_eq!(level_order(&build_tree("2 N 1 3 N")), vec![2, 1, 3]);
        assert_eq!(level_order(&build_tree("1 2 N 3 N 4 N")), vec![1, 2, 3, 4]);
        assert_eq!(
            level_order(&build_tree("1 2 3 N N 4 6 N 5 N N 7 N")),
            vec![1, 2, 3, 4, 6, 5, 7]
        );
    }
}
