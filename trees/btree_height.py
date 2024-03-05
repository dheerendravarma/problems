from trees import TreeNode, buid_tree

"""
Test Cases
3
1 2 3 N N N N
2 N 1 N N 3 N N N N N N N
1 2 N 3 N N N 4 N N N N N N N N N
"""
def get_height(root: TreeNode) -> int:
    if root is None:
        return 0
    return max(get_height(root.left), get_height(root.right)) + 1


def main():
    test_cases: int = int(input())
    results = []
    for _ in range(test_cases):
        tree_str: str = input()
        root: TreeNode = buid_tree(tree_str)
        height: int = get_height(root)
        results.append(height)
    for res in results:
        print(res)

if __name__ == "__main__":
    main()
