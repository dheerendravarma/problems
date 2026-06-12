/*
Test Cases
4
1 2 3
2 N 1 3 N
1 2 N 3 N 4 N
1 2 3 N N 4 6 N 5 N N 7 N

Output
1 2 3
2 1 3
1 2 3 4
1 2 3 4 5 7 6
*/

use std::io::{self, BufRead};
use tree::{build_tree, NodeRef, TreeLink};

#[allow(dead_code)]
fn pre_order_recursive(root: &TreeLink, result: &mut Vec<i64>) {
    if let Some(node) = root {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        let data = node.borrow().data;
        result.push(data);
        pre_order_recursive(&left, result);
        pre_order_recursive(&right, result);
    }
}

fn pre_order_iterative(root: &TreeLink) -> Vec<i64> {
    let mut result = Vec::new();
    let mut stack: Vec<NodeRef> = Vec::new();
    if let Some(r) = root.clone() {
        stack.push(r);
    }
    while let Some(node) = stack.pop() {
        result.push(node.borrow().data);
        if let Some(right) = node.borrow().right.clone() {
            stack.push(right);
        }
        if let Some(left) = node.borrow().left.clone() {
            stack.push(left);
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
        let root = build_tree(line.trim());
        results.push(pre_order_iterative(&root));
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
    fn test_preorder_iterative() {
        assert_eq!(pre_order_iterative(&build_tree("1 2 3")), vec![1, 2, 3]);
        assert_eq!(pre_order_iterative(&build_tree("2 N 1 3 N")), vec![2, 1, 3]);
        assert_eq!(
            pre_order_iterative(&build_tree("1 2 N 3 N 4 N")),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            pre_order_iterative(&build_tree("1 2 3 N N 4 6 N 5 N N 7 N")),
            vec![1, 2, 3, 4, 5, 7, 6]
        );
    }

    #[test]
    fn test_preorder_recursive() {
        let mut result = Vec::new();
        pre_order_recursive(&build_tree("1 2 3"), &mut result);
        assert_eq!(result, vec![1, 2, 3]);

        let mut result = Vec::new();
        pre_order_recursive(&build_tree("1 2 3 N N 4 6 N 5 N N 7 N"), &mut result);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 7, 6]);
    }
}
