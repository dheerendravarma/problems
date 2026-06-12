/*
Test Cases
4
1 2 3
2 N 1 3 N
1 2 N 3 N 4 N
1 2 3 N N 4 6 N 5 N N 7 N

Output
2 1 3
2 3 1
4 3 2 1
2 1 4 7 5 3 6
*/

use std::io::{self, BufRead};
use tree::{build_tree, NodeRef, TreeLink};

#[allow(dead_code)]
fn in_order_recursive(root: &TreeLink, result: &mut Vec<i64>) {
    if let Some(node) = root {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        let data = node.borrow().data;
        in_order_recursive(&left, result);
        result.push(data);
        in_order_recursive(&right, result);
    }
}

fn in_order_iterative(root: &TreeLink) -> Vec<i64> {
    let mut result = Vec::new();
    let mut stack: Vec<NodeRef> = Vec::new();
    let mut current: TreeLink = root.clone();

    loop {
        while let Some(node) = current {
            let left = node.borrow().left.clone();
            stack.push(node);
            current = left;
        }
        match stack.pop() {
            None => break,
            Some(node) => {
                result.push(node.borrow().data);
                current = node.borrow().right.clone();
            }
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
        results.push(in_order_iterative(&root));
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
    fn test_inorder_iterative() {
        assert_eq!(in_order_iterative(&build_tree("1 2 3")), vec![2, 1, 3]);
        assert_eq!(in_order_iterative(&build_tree("2 N 1 3 N")), vec![2, 3, 1]);
        assert_eq!(in_order_iterative(&build_tree("1 2 N 3 N 4 N")), vec![4, 3, 2, 1]);
        assert_eq!(
            in_order_iterative(&build_tree("1 2 3 N N 4 6 N 5 N N 7 N")),
            vec![2, 1, 4, 7, 5, 3, 6]
        );
    }

    #[test]
    fn test_inorder_recursive() {
        let mut result = Vec::new();
        in_order_recursive(&build_tree("1 2 3"), &mut result);
        assert_eq!(result, vec![2, 1, 3]);

        let mut result = Vec::new();
        in_order_recursive(&build_tree("1 2 3 N N 4 6 N 5 N N 7 N"), &mut result);
        assert_eq!(result, vec![2, 1, 4, 7, 5, 3, 6]);
    }
}
