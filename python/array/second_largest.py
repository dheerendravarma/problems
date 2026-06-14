def second_largest_naive(numbers):
    unique = sorted(set(numbers))
    return unique[-2] if len(unique) >= 2 else -1

def second_largest_two_pass(numbers):
    if len(numbers) < 2:
        return -1
    largest = max(numbers)
    second_largest = float("-inf")
    for number in numbers:
        if number != largest and number > second_largest:
            second_largest = number
    return second_largest if second_largest != float("-inf") else -1

def second_largest_optimal(numbers):
    if len(numbers) < 2:
        return -1
    first_largest = second_largest = float("-inf")
    for number in numbers:
        if number > first_largest:
            second_largest = first_largest
            first_largest = number
        elif first_largest > number > second_largest:
            second_largest = number
    return second_largest if second_largest != float("-inf") else -1

def main():
    numbers = list(map(int, input().split(" ")))

    # Naive approach
    second_largest = second_largest_naive(numbers)
    print(second_largest)

    # Two-pass approach
    second_largest = second_largest_two_pass(numbers)
    print(second_largest)

    # Optimal approach
    second_largest = second_largest_optimal(numbers)
    print(second_largest)

if __name__ == "__main__":
    main()
