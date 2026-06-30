/*
Test Cases
3 2 3
2 2 1 1 1 2 2 2
1

Output (sort / hash map / Boyer-Moore per input line)
3
3
3
2
2
2
1
1
1
*/

use std::collections::HashMap;
use std::io::{self, BufRead};

// After sorting, the majority element occupies the middle index.
// Time: O(n log n)  Space: O(1)
fn majority_element_sort(numbers: &mut [i64]) -> i64 {
    numbers.sort();
    numbers[numbers.len() / 2]
}

// Tally counts; return as soon as any value exceeds n // 2.
// Time: O(n)  Space: O(n)
fn majority_element_hash_table(numbers: &[i64]) -> Option<i64> {
    let mut count: HashMap<i64, i32> = HashMap::new();
    let half = (numbers.len() / 2) as i32;
    for &num in numbers {
        let entry = count.entry(num).or_insert(0);
        *entry += 1;
        if *entry > half {
            return Some(num);
        }
    }
    None
}

// Maintain a candidate and a balance; mismatches cancel out.
// Time: O(n)  Space: O(1)
fn majority_element_boyer_moore(numbers: &[i64]) -> Option<i64> {
    let mut balance: i32 = 0;
    let mut candidate: Option<i64> = None;
    for &num in numbers {
        if balance == 0 {
            candidate = Some(num);
        }
        if Some(num) == candidate {
            balance += 1;
        } else {
            balance -= 1;
        }
    }
    candidate
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let input: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut numbers = input.clone();
    println!("{}", majority_element_sort(&mut numbers));

    println!("{}", majority_element_hash_table(&input).unwrap());

    println!("{}", majority_element_boyer_moore(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut nums = vec![3, 2, 3];
        assert_eq!(majority_element_sort(&mut nums), 3);

        let mut nums = vec![2, 2, 1, 1, 1, 2, 2, 2];
        assert_eq!(majority_element_sort(&mut nums), 2);
    }

    #[test]
    fn test_hash_table() {
        assert_eq!(majority_element_hash_table(&[3, 2, 3]), Some(3));
        assert_eq!(
            majority_element_hash_table(&[2, 2, 1, 1, 1, 2, 2, 2]),
            Some(2)
        );
    }

    #[test]
    fn test_boyer_moore() {
        assert_eq!(majority_element_boyer_moore(&[3, 2, 3]), Some(3));
        assert_eq!(
            majority_element_boyer_moore(&[2, 2, 1, 1, 1, 2, 2, 2]),
            Some(2)
        );
    }
}
