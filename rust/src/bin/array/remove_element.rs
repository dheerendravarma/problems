/*
Test Cases
3 2 2 3
3
0 1 2 2 3 0 4 2
2
1 1 1
1

Output (naive / two pointers per input pair)
2 2
2 2
0 1 3 0 4
0 1 3 0 4

0
0
*/

use std::io::{self, BufRead};

// Filter into a new Vec, then copy back into the original buffer.
// Time: O(n)  Space: O(n)
fn remove_element_naive(numbers: &mut [i64], value: i64) -> usize {
    let new_nums: Vec<i64> = numbers.iter().copied().filter(|&n| n != value).collect();
    let len = new_nums.len();
    numbers[..len].copy_from_slice(&new_nums);
    len
}

// read scans every index; write tracks the next slot for a kept value.
// Time: O(n)  Space: O(1)
fn remove_element_two_pointers(numbers: &mut [i64], value: i64) -> usize {
    let nums_length = numbers.len();
    let mut idx: usize = 0;
    let mut current: usize = 0;
    while current < nums_length {
        if numbers[current] != value {
            numbers[idx] = numbers[current];
            idx += 1;
        }
        current += 1;
    }
    idx
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut numbers: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();

    let input_copy: Vec<i64> = numbers.clone();
    let k = remove_element_naive(&mut numbers, value);
    let strs: Vec<String> = numbers[..k].iter().map(|v| v.to_string()).collect();
    println!("{}", strs.join(" "));

    numbers = input_copy;
    let k = remove_element_two_pointers(&mut numbers, value);
    let strs: Vec<String> = numbers[..k].iter().map(|v| v.to_string()).collect();
    println!("{}", strs.join(" "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(remove_element_naive(&mut nums, 3), 2);
        assert_eq!(&nums[..2], &[2, 2]);

        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(remove_element_naive(&mut nums, 2), 5);
        assert_eq!(&nums[..5], &[0, 1, 3, 0, 4]);

        let mut nums = vec![1, 1, 1];
        assert_eq!(remove_element_naive(&mut nums, 1), 0);
    }

    #[test]
    fn test_two_pointers() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(remove_element_two_pointers(&mut nums, 3), 2);
        assert_eq!(&nums[..2], &[2, 2]);

        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(remove_element_two_pointers(&mut nums, 2), 5);
        assert_eq!(&nums[..5], &[0, 1, 3, 0, 4]);

        let mut nums = vec![1, 1, 1];
        assert_eq!(remove_element_two_pointers(&mut nums, 1), 0);
    }
}
