/*
Test Cases
10 10 9
5 1 5 2 3 2
1 2

Output (naive / two-pass / optimal per input line)
9
9
9
*/

use std::io::{self, BufRead};

// Sort ascending, then scan backwards past all copies of the largest
// to find the first strictly smaller value — the second distinct maximum.
// Time: O(n log n)  Space: O(n)
fn second_largest_naive(numbers: &[i64]) -> i64 {
    if numbers.len() < 2 {
        return -1;
    }
    let mut sorted: Vec<i64> = numbers.to_vec();
    sorted.sort();
    let largest: i64 = *sorted.last().unwrap();
    for &val in sorted.iter().rev().skip(1) {
        if val != largest {
            return val;
        }
    }
    -1
}

// Two-pass: find max first, then scan for the largest value strictly below it
fn second_largest_two_pass(numbers: &[i64]) -> i64 {
    if numbers.len() < 2 {
        return -1;
    }
    let largest = *numbers.iter().max().unwrap();
    let mut second_largest = i64::MIN;
    for &number in numbers {
        if number != largest && number > second_largest {
            second_largest = number;
        }
    }
    if second_largest == i64::MIN {
        -1
    } else {
        second_largest
    }
}

// Optimal: single pass tracking first and second distinct maximums
fn second_largest_optimal(numbers: &[i64]) -> i64 {
    if numbers.len() < 2 {
        return -1;
    }
    let mut first_largest = i64::MIN;
    let mut second_largest = i64::MIN;
    for &number in numbers {
        if number > first_largest {
            second_largest = first_largest;
            first_largest = number;
        } else if first_largest > number && number > second_largest {
            second_largest = number;
        }
    }
    if second_largest == i64::MIN {
        -1
    } else {
        second_largest
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", second_largest_naive(&numbers));
    println!("{}", second_largest_two_pass(&numbers));
    println!("{}", second_largest_optimal(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive() {
        assert_eq!(second_largest_naive(&[10, 10, 9]), 9);
        assert_eq!(second_largest_naive(&[5, 1, 5, 2, 3, 2]), 3);
        assert_eq!(second_largest_naive(&[1, 2]), 1);
        assert_eq!(second_largest_naive(&[3, 1, 4, 1, 5, 9, 2, 6]), 6);
    }

    #[test]
    fn test_two_pass() {
        assert_eq!(second_largest_two_pass(&[10, 10, 9]), 9);
        assert_eq!(second_largest_two_pass(&[5, 1, 5, 2, 3, 2]), 3);
        assert_eq!(second_largest_two_pass(&[1, 2]), 1);
        assert_eq!(second_largest_two_pass(&[3, 1, 4, 1, 5, 9, 2, 6]), 6);
    }

    #[test]
    fn test_optimal() {
        assert_eq!(second_largest_optimal(&[10, 10, 9]), 9);
        assert_eq!(second_largest_optimal(&[5, 1, 5, 2, 3, 2]), 3);
        assert_eq!(second_largest_optimal(&[1, 2]), 1);
        assert_eq!(second_largest_optimal(&[3, 1, 4, 1, 5, 9, 2, 6]), 6);
    }
}
