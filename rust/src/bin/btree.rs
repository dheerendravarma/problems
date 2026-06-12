/*
Test Cases
4
1 2 3
2 N 1 3 N
1 2 N 3 N 4 N
1 2 3 N N 4 6 N 5 N N 7 N
*/

use std::io::{self, BufRead};
use tree::build_tree;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let root = build_tree(line.trim()).expect("empty tree");
        println!("{}", root.borrow().data);
    }
}
