from trees import TreeNode, build_tree

"""
Test Cases
4
1 2 3
2 N 1 3 N
1 2 N 3 N 4 N
1 2 3 N N 4 6 N 5 N N 7 N

Output
2
3
4
5
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
        root: TreeNode = build_tree(tree_str)
        height: int = get_height(root)
        results.append(height)
    for res in results:
        print(res)

if __name__ == "__main__":
    main()
