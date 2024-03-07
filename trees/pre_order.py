from trees import TreeNode, buid_tree_iter
from typing import List


def pre_order(root: TreeNode) -> List:
    if root is None:
        return []
    return [root.data] + pre_order(root.left) + pre_order(root.right)

def main():
    test_cases: int = int(input())
    for _ in range(test_cases):
        tree_str = input()
        root = build_tree_iter(tree_str)
        pre_order_list = pre_order(root)
        print(" ".join(pre_order_list))


if __name__ == "__main__":
    main()
