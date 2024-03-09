from trees import TreeNode, build_tree


"""
Test Cases
4
1 2 3
2 N 1 3 N
1 2 N 3 N 4 N
1 2 3 N N 4 6 N 5 N N 7 N
"""


def main():
    test_cases: int = int(input())
    for _ in range(test_cases):
        tree_str = input()
        root: TreeNode = build_tree(tree_str)
        print(root.data)


if __name__ == "__main__":
    main()
