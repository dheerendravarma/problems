"""
Problem 7: Longest Common Prefix

Write a function to find the longest common prefix string amongst an array of strings.
If there is no common prefix, return an empty string.

Approaches:
1. Vertical scan — shrink a running prefix until every string starts with it: O(n·m) time, O(1) space
2. Sort — sort lexicographically, then compare the first and last strings: O(n·m log n) time, O(n·m) space
"""


def longest_common_prefix_naive(strings: list[str]) -> str:
    # Start with strings[0] as prefix; shrink from the end until all strings match.
    # Time: O(n·m)  Space: O(1)
    if not strings:
        return ""
    strings_length: int = len(strings)
    if strings_length == 1:
        return strings[0]

    common_prefix: str = strings[0]
    string: str
    for string in strings[1:]:
        while not string.startswith(common_prefix):
            common_prefix = common_prefix[:-1]
            if not common_prefix:
                return ""
    return common_prefix


def longest_common_optimal(strings: list[str]) -> str:
    # Sort lexicographically; LCP of all strings equals LCP of sorted endpoints.
    # Time: O(n·m log n)  Space: O(n·m)
    if not strings:
        return ""
    strings_length: int = len(strings)
    if strings_length == 1:
        return strings[0]

    sorted_strings: list[str] = sorted(strings)
    first: str = sorted_strings[0]
    last: str = sorted_strings[-1]
    first_length: int = len(first)
    last_length: int = len(last)

    common_prefix: str = ""
    idx: int
    for idx in range(first_length):
        if idx < last_length and first[idx] == last[idx]:
            common_prefix += first[idx]
        else:
            break
    return common_prefix


def main() -> None:
    strings: list[str] = list(input().split(" "))

    common_prefix: str = longest_common_prefix_naive(strings)
    print(common_prefix)

    common_prefix = longest_common_optimal(strings)
    print(common_prefix)


if __name__ == "__main__":
    main()
