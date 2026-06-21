def second_largest_naive(numbers: list[int]) -> int:
    # Deduplicate so equal copies of the largest don't shadow the true second
    # distinct maximum, then sort ascending. The second-largest is at index -2.
    # Time: O(n log n)  Space: O(n)
    unique_numbers: list[int] = sorted(set(numbers))
    return unique_numbers[-2] if len(unique_numbers) >= 2 else -1


def second_largest_two_pass(numbers: list[int]) -> int:
    # Time: O(n)  Space: O(1)
    numbers_length: int = len(numbers)
    if numbers_length < 2:
        return -1

    # Pass 1: find the largest value.
    first_largest: int = max(numbers)

    # Pass 2: find the largest value that is strictly less than first_largest.
    # -inf ensures any real integer immediately beats it on the first hit.
    second_largest: int | float = float("-inf")
    number: int
    for number in numbers:
        if number != first_largest and number > second_largest:
            second_largest = number

    # If second_largest was never updated, all elements are equal — no valid answer.
    return second_largest if second_largest != float("-inf") else -1


def second_largest_optimal(numbers: list[int]) -> int:
    # Single pass: maintain the top-two distinct values simultaneously.
    # Time: O(n)  Space: O(1)
    numbers_length: int = len(numbers)
    if numbers_length < 2:
        return -1

    first_largest: int | float = float("-inf")
    second_largest: int | float = float("-inf")
    number: int
    for number in numbers:
        if number > first_largest:
            # New overall maximum: demote the old first to second.
            second_largest = first_largest
            first_largest = number
        elif first_largest > number > second_largest:
            # Not a new maximum but beats the current second — update second only.
            second_largest = number

    return second_largest if second_largest != float("-inf") else -1


def main() -> None:
    numbers: list[int] = list(map(int, input().split(" ")))

    # Naive approach
    second_largest: int = second_largest_naive(numbers)
    print(second_largest)

    # Two-pass approach
    second_largest = second_largest_two_pass(numbers)
    print(second_largest)

    # Optimal approach
    second_largest = second_largest_optimal(numbers)
    print(second_largest)


if __name__ == "__main__":
    main()
