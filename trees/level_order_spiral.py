from trees import TreeNode, build_tree

"""
Test Cases
4
1 2 3
2 N 1 3 N
1 2 N 3 N 4 N
1 2 3 N N 4 6 N 5 N N 7 N

Output
1 2 3
2 1 3
1 2 3 4
1 2 3 6 4 5 7
"""


def level_order_spiral(root: TreeNode) -> list:
    if root is None:
        return []
    queue = [root]
    spiral_order = []
    level = 0

    while queue:
        current_level_nodes = [node.data for node in queue]

        if level % 2 == 0:
            spiral_order.extend(current_level_nodes[::-1])
        else:
            spiral_order.extend(current_level_nodes)

        next_level = []
        for node in queue:
            if node.left:
                next_level.append(node.left)
            if node.right:
                next_level.append(node.right)

        queue = next_level
        level += 1
    return spiral_order


def main():
    test_cases: int = int(input())
    results = []
    for _ in range(test_cases):
        tree_str: str = input()
        root: TreeNode = build_tree(tree_str)
        level_order_list: list = level_order_spiral(root)
        results.append(level_order_list)

    for res in results:
        print(" ".join(res))


if __name__ == "__main__":
    main()
