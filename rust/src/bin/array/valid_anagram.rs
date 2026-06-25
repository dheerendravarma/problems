/*
Test Cases
anagram
nagaram
rat
car
a
a

Output (sort / hash map / count array per input pair)
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

// Sort both strings; equal multisets have identical sorted forms.
// Time: O(n log n)  Space: O(n)
fn is_anagram_sort(s: &str, t: &str) -> bool {
    let mut s_sorted: Vec<char> = s.chars().collect();
    let mut t_sorted: Vec<char> = t.chars().collect();
    s_sorted.sort();
    t_sorted.sort();
    s_sorted == t_sorted
}

// Build a frequency map for each string, then compare counts per character.
// Time: O(n)  Space: O(n)
fn is_anagram_hash_map(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_counter: HashMap<char, i32> = HashMap::new();
    let mut t_counter: HashMap<char, i32> = HashMap::new();

    for ch in s.chars() {
        *s_counter.entry(ch).or_insert(0) += 1;
    }
    for ch in t.chars() {
        *t_counter.entry(ch).or_insert(0) += 1;
    }

    for (ch, count) in s_counter {
        if t_counter.get(&ch).copied().unwrap_or(0) != count {
            return false;
        }
    }
    true
}

// Single pass: increment for s[i], decrement for t[i] in a 26-slot table.
// Time: O(n)  Space: O(1)
fn is_anagram_count_array(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let mut counter: [i32; 26] = [0; 26];

    for idx in 0..s_chars.len() {
        counter[(s_chars[idx] as u8 - b'a') as usize] += 1;
        counter[(t_chars[idx] as u8 - b'a') as usize] -= 1;
    }

    counter.iter().all(|&c| c == 0)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let s = lines.next().unwrap().unwrap();
    let t = lines.next().unwrap().unwrap();

    println!("{}", is_anagram_sort(&s, &t));
    println!("{}", is_anagram_hash_map(&s, &t));
    println!("{}", is_anagram_count_array(&s, &t));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        assert!(is_anagram_sort("anagram", "nagaram"));
        assert!(!is_anagram_sort("rat", "car"));
        assert!(is_anagram_sort("a", "a"));
    }

    #[test]
    fn test_hash_map() {
        assert!(is_anagram_hash_map("anagram", "nagaram"));
        assert!(!is_anagram_hash_map("rat", "car"));
        assert!(is_anagram_hash_map("a", "a"));
    }

    #[test]
    fn test_count_array() {
        assert!(is_anagram_count_array("anagram", "nagaram"));
        assert!(!is_anagram_count_array("rat", "car"));
        assert!(is_anagram_count_array("a", "a"));
    }
}
