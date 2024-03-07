from trees import TreeNode, buid_tree_iter
from typing import List

"""
Test Cases
1
1 2 3 N N N N
"""
def post_order(root: TreeNode) -> List:
    if root is None:
        return []
    return post_order(root.left) + post_order(root.right) + [root.data]


def main():
    test_cases: int = int(input())
    for _ in range(test_cases):
        tree_str: str = input()
        root: TreeNode = build_tree_iter(tree_str)
        postorder_list: List = post_order(root)
        print(" ".join(postorder_list))

if __name__ == "__main__":
    main()
