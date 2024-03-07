from trees import TreeNode, build_tree_iter


"""
Test Cases
1
1 2 3 4 5 N N N N N N
"""
def main():
    test_cases: int = int(input())
    for _ in range(test_cases):
        tree_str = input()
        root: TreeNode = build_tree_iter(tree_str)
        print(root.data)

if __name__ == "__main__":
    main()
