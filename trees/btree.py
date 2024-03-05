from trees import TreeNode, buid_tree


"""
Test Cases
1
1 2 3 4 5 N N N N N N
"""
def main():
    test_cases: int = int(input())
    for _ in range(test_cases):
        tree_str = input()
        root: TreeNode = buid_tree(tree_str)
        print(root.data)

if __name__ == "__main__":
    main()
