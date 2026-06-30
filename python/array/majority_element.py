"""
Problem 9: Majority Element

Given an array of size n, return the element that appears more than ⌊n / 2⌋ times.
The majority element always exists in the input.

Approaches:
1. Sort — sort and take the middle index: O(n log n) time, O(1) space
2. Hash map — count frequencies, return when count > n/2: O(n) time, O(n) space
3. Boyer-Moore — cancel votes in one pass: O(n) time, O(1) space
"""


def majority_element_sort(numbers: list[int]) -> int:
    # After sorting, the majority element occupies the middle index.
    # Time: O(n log n)  Space: O(1)
    numbers.sort()
    return numbers[len(numbers) // 2]


def majority_element_hash_table(numbers: list[int]) -> int | None:
    # Tally counts; return as soon as any value exceeds n // 2.
    # Time: O(n)  Space: O(n)
    count: dict[int, int] = {}
    num: int
    for num in numbers:
        count[num] = count.get(num, 0) + 1
        if count[num] > len(numbers) // 2:
            return num
    return None


def majority_element_boyer_moore(numbers: list[int]) -> int | None:
    # Maintain a candidate and a balance; mismatches cancel out.
    # Time: O(n)  Space: O(1)
    balance: int = 0
    candidate: int | None = None
    num: int
    for num in numbers:
        if balance == 0:
            candidate = num
        balance += 1 if num == candidate else -1
    return candidate


def main() -> None:
    numbers_input: list[int] = list(map(int, input().split(" ")))

    numbers: list[int] = numbers_input.copy()
    majority_element: int = majority_element_sort(numbers)
    print(majority_element)

    numbers = numbers_input.copy()
    result = majority_element_hash_table(numbers)
    print(result)

    numbers = numbers_input.copy()
    result = majority_element_boyer_moore(numbers)
    print(result)


if __name__ == "__main__":
    main()
