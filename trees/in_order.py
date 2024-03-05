from trees import TreeNode, buid_tree
from typing import List

"""
Test Cases
1
1 2 3 N N N N
"""
def in_order(root: TreeNode) -> List:
    if root is None:
        return []
    return in_order(root.left) + [root.data] + in_order(root.right)


def main():
    test_cases: int = int(input())
    for _ in range(test_cases):
        tree_str: str = input()
        root: TreeNode = buid_tree(tree_str)
        inorder_list: List = in_order(root)
        print(" ".join(inorder_list))

if __name__ == "__main__":
    main()
