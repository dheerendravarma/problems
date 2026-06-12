/*
Test Cases
4
1 2 3
2 N 1 3 N
1 2 N 3 N 4 N
1 2 3 N N 4 6 N 5 N N 7 N
*/

use std::io::{self, BufRead};
use tree::{build_tree, TreeLink};

fn get_height(root: &TreeLink) -> usize {
    match root {
        None => 0,
        Some(node) => {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            1 + get_height(&left).max(get_height(&right))
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut results = Vec::new();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let root = build_tree(line.trim());
        results.push(get_height(&root));
    }
    for h in &results {
        println!("{}", h);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree::build_tree;

    #[test]
    fn test_height() {
        assert_eq!(get_height(&build_tree("1 2 3")), 2);
        assert_eq!(get_height(&build_tree("2 N 1 3 N")), 3);
        assert_eq!(get_height(&build_tree("1 2 N 3 N 4 N")), 4);
        assert_eq!(get_height(&build_tree("1 2 3 N N 4 6 N 5 N N 7 N")), 5);
    }
}
