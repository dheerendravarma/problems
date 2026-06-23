/*
Test Cases
1 2 3 4
1 2
3 1 4 1 5 9 2 6

Output
1 2 3 4 1 2 3 4
1 2 1 2
3 1 4 1 5 9 2 6 3 1 4 1 5 9 2 6
*/

use std::io::{self, BufRead};

// Allocate a 2n Vec and copy numbers into both halves:
// positions [0, n) and positions [n, 2n) each receive the full slice.
// Time: O(n)  Space: O(n)
fn concatenate_arrays(numbers: &[i64]) -> Vec<i64> {
    let numbers_length: usize = numbers.len();
    let mut concatenated_array: Vec<i64> = vec![0; 2 * numbers_length];
    concatenated_array[..numbers_length].copy_from_slice(numbers);
    concatenated_array[numbers_length..].copy_from_slice(numbers);
    concatenated_array
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result: Vec<i64> = concatenate_arrays(&numbers);
    let strs: Vec<String> = result.iter().map(|v| v.to_string()).collect();
    println!("{}", strs.join(" "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate_arrays() {
        assert_eq!(
            concatenate_arrays(&[1, 2, 3, 4]),
            vec![1, 2, 3, 4, 1, 2, 3, 4]
        );
        assert_eq!(concatenate_arrays(&[1, 2]), vec![1, 2, 1, 2]);
        assert_eq!(
            concatenate_arrays(&[3, 1, 4, 1, 5, 9, 2, 6]),
            vec![3, 1, 4, 1, 5, 9, 2, 6, 3, 1, 4, 1, 5, 9, 2, 6]
        );
    }
}
