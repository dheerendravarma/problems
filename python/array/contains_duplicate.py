"""
Problem 4: Contains Duplicate

Given an integer array, return True if any value appears at least twice.

Approaches:
1. Naive — compare every pair with nested loops: O(n²) time, O(1) space
2. Sort — sort ascending, then scan adjacent neighbors: O(n log n) time, O(n) space
3. Hash map — single pass with a frequency map: O(n) time, O(n) space
"""


def is_having_duplicate_naive(numbers: list[int]) -> bool:
    # Compare every unordered pair (i, j) where i < j.
    # Time: O(n²)  Space: O(1)
    numbers_length: int = len(numbers)
    f_idx: int
    s_idx: int
    for f_idx in range(numbers_length):
        for s_idx in range(f_idx + 1, numbers_length):
            if numbers[f_idx] == numbers[s_idx]:
                return True
    return False


def is_having_duplicate_sort(numbers: list[int]) -> bool:
    # Sort ascending; duplicates become adjacent neighbors.
    # Time: O(n log n)  Space: O(n)
    sorted_numbers: list[int] = sorted(numbers)
    idx: int
    for idx in range(len(sorted_numbers) - 1):
        if sorted_numbers[idx] == sorted_numbers[idx + 1]:
            return True
    return False


def is_having_duplicate_hash_map(numbers: list[int]) -> bool:
    # Single pass: return True on first repeat seen.
    # Time: O(n)  Space: O(n)
    frequency_counter: dict[int, int] = {}
    num: int
    for num in numbers:
        if num in frequency_counter:
            return True
        frequency_counter[num] = 1
    return False


def main() -> None:
    numbers: list[int] = list(map(int, input().split(" ")))

    is_duplicate: bool = is_having_duplicate_naive(numbers)
    print(is_duplicate)

    is_duplicate = is_having_duplicate_sort(numbers)
    print(is_duplicate)

    is_duplicate = is_having_duplicate_hash_map(numbers)
    print(is_duplicate)


if __name__ == "__main__":
    main()
