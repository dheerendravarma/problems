from typing import List

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
1 2 3 4 5 7 6
"""


def pre_order_recursive(root: TreeNode) -> List:
    if root is None:
        return []
    return (
        [root.data] + pre_order_recursive(root.left) + pre_order_recursive(root.right)
    )


def pre_order_iterative(root: TreeNode) -> List:
    if root is None:
        return []
    stack = [root]
    pre_order_list = []
    while stack:
        node = stack.pop(-1)
        pre_order_list.append(node.data)

        if node.right:
            stack.append(node.right)

        if node.left:
            stack.append(node.left)

    return pre_order_list


def main():
    test_cases: int = int(input())
    results = []
    for _ in range(test_cases):
        tree_str: str = input()
        root: TreeNode = build_tree(tree_str)
        inorder_list: List = pre_order_iterative(root)
        results.append(inorder_list)
    for res in results:
        print(" ".join(res))


if __name__ == "__main__":
    main()
