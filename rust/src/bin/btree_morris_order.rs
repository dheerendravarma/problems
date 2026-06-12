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

// Morris in-order traversal: O(1) space by threading the tree.
// The tree is consumed (left links are severed during traversal).
fn morris_order_traversal(root: &TreeLink) -> Vec<i64> {
    let mut result = Vec::new();
    let mut current: TreeLink = root.clone();

    while let Some(curr_node) = current {
        let left: TreeLink = curr_node.borrow().left.clone();

        if left.is_none() {
            result.push(curr_node.borrow().data);
            current = curr_node.borrow().right.clone();
        } else {
            // Find the rightmost node in the left subtree (inorder predecessor).
            let mut prev_node: NodeRef = left.clone().unwrap();
            loop {
                let next: TreeLink = prev_node.borrow().right.clone();
                match next {
                    None => break,
                    Some(n) => prev_node = n,
                }
            }

            // Thread: predecessor.right → current node.
            // RefMut guard drops at the semicolon before the next borrow.
            prev_node.borrow_mut().right = Some(curr_node.clone());

            let new_current: TreeLink = left;

            // Sever the left link so we don't revisit. prev_node and curr_node
            // are distinct Rc instances — no aliasing, no runtime panic.
            curr_node.borrow_mut().left = None;

            current = new_current;
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
        results.push(morris_order_traversal(&root));
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
    fn test_morris_order() {
        assert_eq!(morris_order_traversal(&build_tree("1 2 3")), vec![2, 1, 3]);
        assert_eq!(morris_order_traversal(&build_tree("2 N 1 3 N")), vec![2, 3, 1]);
        assert_eq!(morris_order_traversal(&build_tree("1 2 N 3 N 4 N")), vec![4, 3, 2, 1]);
        assert_eq!(
            morris_order_traversal(&build_tree("1 2 3 N N 4 6 N 5 N N 7 N")),
            vec![2, 1, 4, 7, 5, 3, 6]
        );
    }
}
