from trees import TreeNode, build_tree_iter
from typing import List

"""
Test Cases
2
1 2 3 N N N N
1 2 N 3 N N N 4 N N N N N N N N N
"""
def in_order(root: TreeNode) -> List:
    if root is None:
        return []
    return in_order(root.left) + [root.data] + in_order(root.right)


def main():
    test_cases: int = int(input())
    results = []
    for _ in range(test_cases):
        tree_str: str = input()
        root: TreeNode = build_tree_iter(tree_str)
        inorder_list: List = in_order(root)
        results.append(inorder_list)
    for res in results:
        print(" ".join(res))

if __name__ == "__main__":
    main()
