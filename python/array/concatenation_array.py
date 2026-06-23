def concatenate_arrays(numbers: list[int]) -> list[int]:
    # Allocate a 2n array and fill both halves in a single pass:
    # position i gets numbers[i], position i+n also gets numbers[i].
    # Time: O(n)  Space: O(n)
    numbers_length: int = len(numbers)
    concatenated_array: list[int] = [0] * (2 * numbers_length)
    idx: int
    for idx in range(numbers_length):
        concatenated_array[idx] = numbers[idx]
        concatenated_array[idx + numbers_length] = numbers[idx]
    return concatenated_array


def main() -> None:
    numbers: list[int] = list(map(int, input().split(" ")))

    concatenated_array: list[int] = concatenate_arrays(numbers)
    print(" ".join(map(str, concatenated_array)))


if __name__ == "__main__":
    main()
