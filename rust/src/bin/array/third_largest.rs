/*
Test Cases (all three approaches produce the same value per input)
2 4 1 3 5         → 3   standard case
10 2              → -1  length guard
5 5 5             → 5   all equal, duplicates cascade
10 10 9           → 9   duplicate at the top
100 100 100 99 98 → 100 repeated max fills ranks 1–3
3 1 2             → 1   exactly three elements
-1 -2 -3          → -3  all negative
*/

use std::io::{self, BufRead};

// Sort ascending; the third-largest is at index len-3.
// Duplicates are kept, so equal values fill consecutive rank slots.
// Time: O(n log n)  Space: O(n)
fn third_largest_naive(numbers: &[i64]) -> i64 {
    if numbers.len() < 3 {
        return -1;
    }
    let mut sorted: Vec<i64> = numbers.to_vec();
    sorted.sort();
    sorted[sorted.len() - 3]
}

// Three independent scans, each finding the next rank by skipping the
// *index* (not the value) of the previously found rank.
// Skipping by index means duplicate values at different positions are
// valid candidates for successive ranks.
// Time: O(n)  Space: O(1)
fn third_largest_three_pass(numbers: &[i64]) -> i64 {
    if numbers.len() < 3 {
        return -1;
    }

    let mut first = i64::MIN;
    let mut second = i64::MIN;
    let mut third = i64::MIN;
    let mut first_idx: Option<usize> = None;
    let mut second_idx: Option<usize> = None;
    let mut third_idx: Option<usize> = None;

    // Pass 1: find the overall maximum and its first-occurrence index.
    for (idx, &number) in numbers.iter().enumerate() {
        if number > first {
            first = number;
            first_idx = Some(idx);
        }
    }

    // Pass 2: find the maximum while skipping first_idx.
    for (idx, &number) in numbers.iter().enumerate() {
        if Some(idx) == first_idx {
            continue;
        }
        if number > second {
            second = number;
            second_idx = Some(idx);
        }
    }

    // Pass 3: find the maximum while skipping both first_idx and second_idx.
    for (idx, &number) in numbers.iter().enumerate() {
        if Some(idx) == first_idx || Some(idx) == second_idx {
            continue;
        }
        if number > third {
            third = number;
            third_idx = Some(idx);
        }
    }

    if third_idx.is_some() { third } else { -1 }
}

// Single pass: maintain three rank variables with strict > comparisons.
// Duplicate values naturally cascade into lower rank slots via the
// else-if chain because i64::MIN is always beaten first.
// Time: O(n)  Space: O(1)
fn third_largest_optimal(numbers: &[i64]) -> i64 {
    if numbers.len() < 3 {
        return -1;
    }

    let mut first = i64::MIN;
    let mut second = i64::MIN;
    let mut third = i64::MIN;

    for &number in numbers {
        if number > first {
            // New overall maximum: demote first → second, second → third.
            third = second;
            second = first;
            first = number;
        } else if number > second {
            // Beats second but not first: demote second → third only.
            third = second;
            second = number;
        } else if number > third {
            // Beats only the current third.
            third = number;
        }
    }

    if third == i64::MIN { -1 } else { third }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", third_largest_naive(&numbers));
    println!("{}", third_largest_three_pass(&numbers));
    println!("{}", third_largest_optimal(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive() {
        assert_eq!(third_largest_naive(&[2, 4, 1, 3, 5]), 3);
        assert_eq!(third_largest_naive(&[10, 2]), -1);
        assert_eq!(third_largest_naive(&[5, 5, 5]), 5);
        assert_eq!(third_largest_naive(&[10, 10, 9]), 9);
        assert_eq!(third_largest_naive(&[100, 100, 100, 99, 98]), 100);
        assert_eq!(third_largest_naive(&[3, 1, 2]), 1);
        assert_eq!(third_largest_naive(&[-1, -2, -3]), -3);
    }

    #[test]
    fn test_three_pass() {
        assert_eq!(third_largest_three_pass(&[2, 4, 1, 3, 5]), 3);
        assert_eq!(third_largest_three_pass(&[10, 2]), -1);
        assert_eq!(third_largest_three_pass(&[5, 5, 5]), 5);
        assert_eq!(third_largest_three_pass(&[10, 10, 9]), 9);
        assert_eq!(third_largest_three_pass(&[100, 100, 100, 99, 98]), 100);
        assert_eq!(third_largest_three_pass(&[3, 1, 2]), 1);
        assert_eq!(third_largest_three_pass(&[-1, -2, -3]), -3);
    }

    #[test]
    fn test_optimal() {
        assert_eq!(third_largest_optimal(&[2, 4, 1, 3, 5]), 3);
        assert_eq!(third_largest_optimal(&[10, 2]), -1);
        assert_eq!(third_largest_optimal(&[5, 5, 5]), 5);
        assert_eq!(third_largest_optimal(&[10, 10, 9]), 9);
        assert_eq!(third_largest_optimal(&[100, 100, 100, 99, 98]), 100);
        assert_eq!(third_largest_optimal(&[3, 1, 2]), 1);
        assert_eq!(third_largest_optimal(&[-1, -2, -3]), -3);
    }
}
