from trees import TreeNode, build_tree
from typing import List

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
def in_order(root: TreeNode) -> List:
    if root is None:
        return []
    return in_order(root.left) + [root.data] + in_order(root.right)


def main():
    test_cases: int = int(input())
    results = []
    for _ in range(test_cases):
        tree_str: str = input()
        root: TreeNode = build_tree(tree_str)
        inorder_list: List = in_order(root)
        results.append(inorder_list)
    for res in results:
        print(" ".join(res))

if __name__ == "__main__":
    main()
