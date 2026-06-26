"""
Problem 6: Two Sum

Given an array of integers and a target, return the indices of the two numbers
that add up to the target.

Approaches:
1. Naive — nested loops over every pair: O(n²) time, O(1) space
2. Two-pass hash table — build an index map, then scan for complements: O(n) time, O(n) space
3. One-pass hash table — single scan with complement lookup: O(n) time, O(n) space
"""


def sum_pair_naive(numbers: list[int], target: int) -> list[str]:
    # Compare every unordered pair (i, j) where i < j.
    # Time: O(n²)  Space: O(1)
    numbers_length: int = len(numbers)
    if numbers_length < 2:
        return []

    f_idx: int
    s_idx: int
    for f_idx in range(numbers_length - 1):
        for s_idx in range(f_idx + 1, numbers_length):
            if numbers[f_idx] + numbers[s_idx] == target:
                return [str(f_idx), str(s_idx)]
    return []


def sum_pair_two_pass_hash_table(numbers: list[int], target: int) -> list[str]:
    # Pass 1: map each value to its index. Pass 2: hunt for target − num.
    # Time: O(n)  Space: O(n)
    numbers_length: int = len(numbers)
    if numbers_length < 2:
        return []

    num_to_index: dict[int, int] = {}
    idx: int
    num: int
    for idx, num in enumerate(numbers):
        num_to_index[num] = idx

    for idx, num in enumerate(numbers):
        complement: int = target - num
        if complement in num_to_index and num_to_index[complement] != idx:
            return [str(idx), str(num_to_index[complement])]
    return []


def sum_pair_one_pass_hash_table(numbers: list[int], target: int) -> list[str]:
    # Single pass: check complement before inserting the current value.
    # Time: O(n)  Space: O(n)
    numbers_length: int = len(numbers)
    if numbers_length < 2:
        return []

    num_to_index: dict[int, int] = {}
    idx: int
    num: int
    for idx, num in enumerate(numbers):
        complement: int = target - num
        if complement in num_to_index:
            return [str(num_to_index[complement]), str(idx)]
        num_to_index[num] = idx
    return []


def main() -> None:
    numbers: list[int] = list(map(int, input().split(" ")))
    target: int = int(input())

    sum_pair: list[str] = sum_pair_naive(numbers, target)
    print(" ".join(sum_pair))

    sum_pair = sum_pair_two_pass_hash_table(numbers, target)
    print(" ".join(sum_pair))

    sum_pair = sum_pair_one_pass_hash_table(numbers, target)
    print(" ".join(sum_pair))


if __name__ == "__main__":
    main()
