/*
Test Cases
flower flow flight
dog racecar car
a
bac abc bca
aaa aa aaa

Output (naive / optimal per input line)
fl
fl
"" (empty line)
a
a
"" (empty line)
*/

use std::io::{self, BufRead};

// Start with strings[0] as prefix; shrink from the end until all strings match.
// Time: O(n·m)  Space: O(1)
fn longest_common_prefix_naive(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    if strings.len() == 1 {
        return strings[0].clone();
    }

    let mut common_prefix = strings[0].clone();
    for string in strings.iter().skip(1) {
        while !string.starts_with(&common_prefix) {
            common_prefix.pop();
            if common_prefix.is_empty() {
                return String::new();
            }
        }
    }
    common_prefix
}

// Sort lexicographically; LCP of all strings equals LCP of sorted endpoints.
// Time: O(n·m log n)  Space: O(n·m)
fn longest_common_optimal(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    if strings.len() == 1 {
        return strings[0].clone();
    }

    let mut sorted_strings: Vec<String> = strings.to_vec();
    sorted_strings.sort();

    let first = sorted_strings[0].as_str();
    let last = sorted_strings.last().unwrap().as_str();
    let first_chars: Vec<char> = first.chars().collect();
    let last_chars: Vec<char> = last.chars().collect();
    let mut common_prefix = String::new();

    for idx in 0..first_chars.len() {
        if idx < last_chars.len() && first_chars[idx] == last_chars[idx] {
            common_prefix.push(first_chars[idx]);
        } else {
            break;
        }
    }
    common_prefix
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let strings: Vec<String> = line.split_whitespace().map(String::from).collect();

    println!("{}", longest_common_prefix_naive(&strings));
    println!("{}", longest_common_optimal(&strings));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strs(values: &[&str]) -> Vec<String> {
        values.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_naive() {
        assert_eq!(
            longest_common_prefix_naive(&strs(&["flower", "flow", "flight"])),
            "fl"
        );
        assert_eq!(
            longest_common_prefix_naive(&strs(&["dog", "racecar", "car"])),
            ""
        );
        assert_eq!(longest_common_prefix_naive(&strs(&["a"])), "a");
        assert_eq!(
            longest_common_prefix_naive(&strs(&["aaa", "aa", "aaa"])),
            "aa"
        );
    }

    #[test]
    fn test_optimal() {
        assert_eq!(
            longest_common_optimal(&strs(&["flower", "flow", "flight"])),
            "fl"
        );
        assert_eq!(
            longest_common_optimal(&strs(&["dog", "racecar", "car"])),
            ""
        );
        assert_eq!(longest_common_optimal(&strs(&["a"])), "a");
        assert_eq!(longest_common_optimal(&strs(&["bac", "abc", "bca"])), "");
        assert_eq!(
            longest_common_optimal(&strs(&["aaa", "aa", "aaa"])),
            "aa"
        );
    }
}
