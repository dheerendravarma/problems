from trees import TreeNode, build_tree
from typing import List

"""
https://www.geeksforgeeks.org/problems/largest-value-in-each-level/1
Test Cases
4
1 2 3
2 N 1 3 N
1 2 N 3 N 4 N
1 2 3 N N 4 6 N 5 N N 7 N

Output
1 3
2 1 3
1 2 3 4
1 3 6 5 7
"""


def get_max_level_values(root: TreeNode) -> List[int]:
    if root is None:
        return 0
    queue = [root]
    result = []

    while queue:
        max_level_value = max([int(node.data) for node in queue])
        result.append(max_level_value)

        next_level_nodes = []
        for node in queue:
            if node.left:
                next_level_nodes.append(node.left)

            if node.right:
                next_level_nodes.append(node.right)

        queue = next_level_nodes
    return result


def main():
    test_cases: int = int(input())
    results = []
    for _ in range(test_cases):
        tree_str: str = input()
        root: TreeNode = build_tree(tree_str)
        max_level: int = get_max_level_values(root)
        results.append(max_level)

    for res in results:
        print(" ".join([str(val) for val in res]))


if __name__ == "__main__":
    main()
