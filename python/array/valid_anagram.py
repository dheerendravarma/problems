"""
Problem 5: Valid Anagram

Given two strings s and t, return True if t is an anagram of s.

Approaches:
1. Sort — sort both strings and compare: O(n log n) time, O(n) space
2. Hash map — count characters in each string, then compare maps: O(n) time, O(n) space
3. Count array — fixed-size frequency table for lowercase a–z: O(n) time, O(1) space
"""


def is_anagram_sort(s: str, t: str) -> bool:
    # Sort both strings; equal multisets have identical sorted forms.
    # Time: O(n log n)  Space: O(n)
    return sorted(s) == sorted(t)


def is_anagram_hash_map(s: str, t: str) -> bool:
    # Build a frequency map for each string, then compare counts per character.
    # Time: O(n)  Space: O(n)
    s_length: int = len(s)
    t_length: int = len(t)

    if s_length != t_length:
        return False

    s_counter: dict[str, int] = {}
    t_counter: dict[str, int] = {}
    ch: str

    for ch in s:
        s_counter[ch] = s_counter.get(ch, 0) + 1
    for ch in t:
        t_counter[ch] = t_counter.get(ch, 0) + 1

    for ch, count in s_counter.items():
        if t_counter[ch] != count:
            return False
    return True


def is_anagram_count_array(s: str, t: str) -> bool:
    # Single pass: increment for s[i], decrement for t[i] in a 26-slot table.
    # Time: O(n)  Space: O(1)
    s_length: int = len(s)
    t_length: int = len(t)
    if s_length != t_length:
        return False
    counter: list[int] = [0] * 26
    idx: int
    for idx in range(len(s)):
        counter[ord(s[idx]) - ord("a")] += 1
        counter[ord(t[idx]) - ord("a")] -= 1
    return all(c == 0 for c in counter)


def main() -> None:
    s: str = input()
    t: str = input()

    is_anagram: bool = is_anagram_sort(s, t)
    print(is_anagram)

    is_anagram = is_anagram_hash_map(s, t)
    print(is_anagram)

    is_anagram = is_anagram_count_array(s, t)
    print(is_anagram)


if __name__ == "__main__":
    main()
