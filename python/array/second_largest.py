def second_largest_naive(numbers: list[int]) -> int:
    unique_numbers: list[int] = sorted(set(numbers))
    return unique_numbers[-2] if len(unique_numbers) >= 2 else -1


def second_largest_two_pass(numbers: list[int]) -> int:
    numbers_length: int = len(numbers)
    if numbers_length < 2:
        return -1
    first_largest: int = max(numbers)
    second_largest: int | float = float("-inf")
    number: int
    for number in numbers:
        if number != first_largest and number > second_largest:
            second_largest = number
    return second_largest if second_largest != float("-inf") else -1


def second_largest_optimal(numbers: list[int]) -> int:
    numbers_length: int = len(numbers)
    if numbers_length < 2:
        return -1
    first_largest: int | float = float("-inf")
    second_largest: int | float = float("-inf")
    number: int
    for number in numbers:
        if number > first_largest:
            second_largest = first_largest
            first_largest = number
        elif first_largest > number > second_largest:
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
