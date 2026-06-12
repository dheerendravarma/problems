/*
Test Cases
4
1 2 3
2 N 1 3 N
1 2 N 3 N 4 N
1 2 3 N N 4 6 N 5 N N 7 N

Output
2 3 1
3 1 2
4 3 2 1
2 7 4 5 6 3 1
*/

use std::io::{self, BufRead};
use tree::{build_tree, TreeLink};

fn post_order(root: &TreeLink, result: &mut Vec<i64>) {
    if let Some(node) = root {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        post_order(&left, result);
        post_order(&right, result);
        result.push(node.borrow().data);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut results: Vec<Vec<i64>> = Vec::new();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let root = build_tree(line.trim());
        let mut res = Vec::new();
        post_order(&root, &mut res);
        results.push(res);
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
    fn test_postorder() {
        let mut result = Vec::new();
        post_order(&build_tree("1 2 3"), &mut result);
        assert_eq!(result, vec![2, 3, 1]);

        let mut result = Vec::new();
        post_order(&build_tree("2 N 1 3 N"), &mut result);
        assert_eq!(result, vec![3, 1, 2]);

        let mut result = Vec::new();
        post_order(&build_tree("1 2 N 3 N 4 N"), &mut result);
        assert_eq!(result, vec![4, 3, 2, 1]);

        let mut result = Vec::new();
        post_order(&build_tree("1 2 3 N N 4 6 N 5 N N 7 N"), &mut result);
        assert_eq!(result, vec![2, 7, 5, 4, 6, 3, 1]);
    }
}
