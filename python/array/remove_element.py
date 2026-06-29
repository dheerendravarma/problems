"""
Problem 8: Remove Element

Remove all instances of val in-place from an integer array and return the new length k.
The first k slots of the array must hold the remaining elements (order preserved).

Approaches:
1. Extra array — filter into a new list, copy back: O(n) time, O(n) space
2. Two pointers — in-place write index with read scan: O(n) time, O(1) space
"""


def remove_element_naive(numbers: list[int], value: int) -> int:
    # Filter into a new list, then copy back into the original buffer.
    # Time: O(n)  Space: O(n)
    new_nums: list[int] = [num for num in numbers if num != value]
    numbers[:] = new_nums
    return len(new_nums)


def remove_element_two_pointers(numbers: list[int], value: int) -> int:
    # read scans every index; write tracks the next slot for a kept value.
    # Time: O(n)  Space: O(1)
    nums_length: int = len(numbers)
    idx: int = 0
    current: int = 0
    while current < nums_length:
        if numbers[current] != value:
            numbers[idx] = numbers[current]
            idx += 1
        current += 1
    return idx


def main() -> None:
    numbers_input: list[int] = list(map(int, input().split(" ")))
    value: int = int(input())

    numbers: list[int] = numbers_input.copy()
    result: int = remove_element_naive(numbers, value)
    print(" ".join(map(str, numbers[:result])))

    numbers = numbers_input.copy()
    result = remove_element_two_pointers(numbers, value)
    print(" ".join(map(str, numbers[:result])))


if __name__ == "__main__":
    main()
