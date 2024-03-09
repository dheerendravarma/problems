from trees import TreeNode, build_tree

"""
Test Cases
4
1 2 3
2 N 1 3 N
1 2 N 3 N 4 N
1 2 3 N N 4 6 N 5 N N 7 N

Output
2
3
4
3
"""


def get_max_level_sum(root: TreeNode) -> int:
    if root is None:
        return 0
    queue = [root]
    max_sum = float("-inf")
    level = 1
    max_level = level
    while queue:
        max_level_sum = sum([int(node.data) for node in queue])

        if max_level_sum > max_sum:
            max_sum = max_level_sum
            max_level = level

        next_level_nodes = []
        for node in queue:
            if node.left:
                next_level_nodes.append(node.left)

            if node.right:
                next_level_nodes.append(node.right)

        queue, level = next_level_nodes, level + 1
    return max_level


def main():
    test_cases: int = int(input())
    results = []
    for _ in range(test_cases):
        tree_str: str = input()
        root: TreeNode = build_tree(tree_str)
        max_level: int = get_max_level_sum(root)
        results.append(max_level)

    for res in results:
        print(res)


if __name__ == "__main__":
    main()
