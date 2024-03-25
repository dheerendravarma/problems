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
2 1 3
2 3 1
4 3 2 1
2 1 4 7 5 3 6
"""


def morris_order_traversal(root: TreeNode) -> List:
    if root is None:
        return []
    morris_order = []
    current = root
    while current:
        if current.left is None:
            morris_order.append(current.data)
            current = current.right
        else:
            prev = current.left
            while prev.right:
                prev = prev.right
            prev.right = current
            temp_node = current
            current = current.left
            temp_node.left = None
    return morris_order


def main():
    test_cases: int = int(input())
    results = []
    for _ in range(test_cases):
        tree_str: str = input()
        root: TreeNode = build_tree(tree_str)
        inorder_list: List = morris_order_traversal(root)
        results.append(inorder_list)
    for res in results:
        print(" ".join(res))


if __name__ == "__main__":
    main()
