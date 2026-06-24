/*
Test Cases
1 2 3 1
1 2 3 4
1 1 1 3 3 4 3 2 4 2

Output (naive / sort / hash map per input line)
True
True
True
False
False
False
True
True
True
*/

use std::collections::HashMap;
use std::io::{self, BufRead};

// Compare every unordered pair (i, j) where i < j.
// Time: O(n²)  Space: O(1)
fn contains_duplicate_naive(numbers: &[i64]) -> bool {
    let numbers_length: usize = numbers.len();
    for f_idx in 0..numbers_length {
        for s_idx in (f_idx + 1)..numbers_length {
            if numbers[f_idx] == numbers[s_idx] {
                return true;
            }
        }
    }
    false
}

// Sort ascending; duplicates become adjacent neighbors.
// Time: O(n log n)  Space: O(n)
fn contains_duplicate_sort(numbers: &[i64]) -> bool {
    let mut sorted_numbers: Vec<i64> = numbers.to_vec();
    sorted_numbers.sort();
    for idx in 0..sorted_numbers.len().saturating_sub(1) {
        if sorted_numbers[idx] == sorted_numbers[idx + 1] {
            return true;
        }
    }
    false
}

// Single pass: return true on first repeat seen.
// Time: O(n)  Space: O(n)
fn contains_duplicate_hash_map(numbers: &[i64]) -> bool {
    let mut frequency_counter: HashMap<i64, u8> = HashMap::new();
    for &num in numbers {
        if frequency_counter.contains_key(&num) {
            return true;
        }
        frequency_counter.insert(num, 1);
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", contains_duplicate_naive(&numbers));
    println!("{}", contains_duplicate_sort(&numbers));
    println!("{}", contains_duplicate_hash_map(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive() {
        assert!(contains_duplicate_naive(&[1, 2, 3, 1]));
        assert!(!contains_duplicate_naive(&[1, 2, 3, 4]));
        assert!(contains_duplicate_naive(&[1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
        assert!(!contains_duplicate_naive(&[1]));
    }

    #[test]
    fn test_sort() {
        assert!(contains_duplicate_sort(&[1, 2, 3, 1]));
        assert!(!contains_duplicate_sort(&[1, 2, 3, 4]));
        assert!(contains_duplicate_sort(&[1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
        assert!(!contains_duplicate_sort(&[1]));
    }

    #[test]
    fn test_hash_map() {
        assert!(contains_duplicate_hash_map(&[1, 2, 3, 1]));
        assert!(!contains_duplicate_hash_map(&[1, 2, 3, 4]));
        assert!(contains_duplicate_hash_map(&[1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
        assert!(!contains_duplicate_hash_map(&[1]));
    }
}
