def third_largest_naive(numbers: list[int]) -> int:
    # Sort ascending; third-largest sits at index -3 (third from the right).
    # Duplicates are kept, so equal values fill consecutive rank slots.
    # Time: O(n log n)  Space: O(n)
    if len(numbers) < 3:
        return -1
    sorted_numbers: list[int] = sorted(numbers)
    return sorted_numbers[-3]


def third_largest_three_pass(numbers: list[int]) -> int:
    # Three independent scans, each finding the next rank by skipping the
    # *index* (not the value) of the previously found rank.
    # Skipping by index — not value — means duplicate values at different
    # positions are valid candidates for successive ranks.
    # Time: O(n)  Space: O(1)
    numbers_length: int = len(numbers)
    if numbers_length < 3:
        return -1

    first_largest: int | float = float("-inf")
    second_largest: int | float = float("-inf")
    third_largest: int | float = float("-inf")
    first_idx: int = -1
    second_idx: int = -1
    third_idx: int = -1
    idx: int
    number: int

    # Pass 1: find the overall maximum and its first-occurrence index.
    for idx, number in enumerate(numbers):
        if number > first_largest:
            first_largest = number
            first_idx = idx

    # Pass 2: find the maximum while skipping first_idx.
    for idx, number in enumerate(numbers):
        if idx == first_idx:
            continue
        if number > second_largest:
            second_largest = number
            second_idx = idx

    # Pass 3: find the maximum while skipping both first_idx and second_idx.
    for idx, number in enumerate(numbers):
        if idx == first_idx or idx == second_idx:
            continue
        if number > third_largest:
            third_largest = number
            third_idx = idx

    return int(third_largest) if third_idx != -1 else -1


def third_largest_optimal(numbers: list[int]) -> int:
    # Single pass: maintain three rank variables with strict > comparisons.
    # Duplicate values naturally cascade into lower rank slots via the
    # elif chain because the float("-inf") sentinel is always beaten first.
    # Time: O(n)  Space: O(1)
    numbers_length: int = len(numbers)
    if numbers_length < 3:
        return -1

    first_largest: int | float = float("-inf")
    second_largest: int | float = float("-inf")
    third_largest: int | float = float("-inf")
    number: int
    for number in numbers:
        if number > first_largest:
            # New overall maximum: demote first → second, second → third.
            third_largest, second_largest, first_largest = second_largest, first_largest, number
        elif number > second_largest:
            # Beats second but not first: demote second → third only.
            third_largest, second_largest = second_largest, number
        elif number > third_largest:
            # Beats only the current third.
            third_largest = number

    return int(third_largest) if third_largest != float("-inf") else -1


def main() -> None:
    numbers: list[int] = list(map(int, input().split(" ")))

    # Naive approach
    third_largest: int = third_largest_naive(numbers)
    print(third_largest)

    # Three-pass approach
    third_largest = third_largest_three_pass(numbers)
    print(third_largest)

    # Optimal approach
    third_largest = third_largest_optimal(numbers)
    print(third_largest)


if __name__ == "__main__":
    main()
