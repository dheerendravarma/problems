/*
Test Cases
2 7 11 15
9
3 2 4
6
3 3
6

Output (naive / two-pass / one-pass per input pair)
0 1
0 1
0 1
1 2
1 2
1 2
0 1
0 1
0 1
*/

use std::collections::HashMap;
use std::io::{self, BufRead};

// Compare every unordered pair (i, j) where i < j.
// Time: O(n²)  Space: O(1)
fn sum_pair_naive(numbers: &[i64], target: i64) -> Vec<usize> {
    let numbers_length: usize = numbers.len();
    if numbers_length < 2 {
        return vec![];
    }

    for f_idx in 0..numbers_length - 1 {
        for s_idx in (f_idx + 1)..numbers_length {
            if numbers[f_idx] + numbers[s_idx] == target {
                return vec![f_idx, s_idx];
            }
        }
    }
    vec![]
}

// Pass 1: map each value to its index. Pass 2: hunt for target − num.
// Time: O(n)  Space: O(n)
fn sum_pair_two_pass_hash_table(numbers: &[i64], target: i64) -> Vec<usize> {
    let numbers_length: usize = numbers.len();
    if numbers_length < 2 {
        return vec![];
    }

    let mut num_to_index: HashMap<i64, usize> = HashMap::new();
    for (idx, &num) in numbers.iter().enumerate() {
        num_to_index.insert(num, idx);
    }

    for (idx, &num) in numbers.iter().enumerate() {
        let complement = target - num;
        if let Some(&comp_idx) = num_to_index.get(&complement) {
            if comp_idx != idx {
                return vec![idx, comp_idx];
            }
        }
    }
    vec![]
}

// Single pass: check complement before inserting the current value.
// Time: O(n)  Space: O(n)
fn sum_pair_one_pass_hash_table(numbers: &[i64], target: i64) -> Vec<usize> {
    let numbers_length: usize = numbers.len();
    if numbers_length < 2 {
        return vec![];
    }

    let mut num_to_index: HashMap<i64, usize> = HashMap::new();
    for (idx, &num) in numbers.iter().enumerate() {
        let complement = target - num;
        if let Some(&comp_idx) = num_to_index.get(&complement) {
            return vec![comp_idx, idx];
        }
        num_to_index.insert(num, idx);
    }
    vec![]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let numbers: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let target: i64 = lines.next().unwrap().unwrap().parse().unwrap();

    for pair in [
        sum_pair_naive(&numbers, target),
        sum_pair_two_pass_hash_table(&numbers, target),
        sum_pair_one_pass_hash_table(&numbers, target),
    ] {
        let strs: Vec<String> = pair.iter().map(|i| i.to_string()).collect();
        println!("{}", strs.join(" "));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive() {
        assert_eq!(sum_pair_naive(&[2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(sum_pair_naive(&[3, 2, 4], 6), vec![1, 2]);
        assert_eq!(sum_pair_naive(&[3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_pass() {
        assert_eq!(sum_pair_two_pass_hash_table(&[2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(sum_pair_two_pass_hash_table(&[3, 2, 4], 6), vec![1, 2]);
        assert_eq!(sum_pair_two_pass_hash_table(&[3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_one_pass() {
        assert_eq!(sum_pair_one_pass_hash_table(&[2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(sum_pair_one_pass_hash_table(&[3, 2, 4], 6), vec![1, 2]);
        assert_eq!(sum_pair_one_pass_hash_table(&[3, 3], 6), vec![0, 1]);
    }
}
