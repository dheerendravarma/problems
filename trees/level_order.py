from trees import TreeNode, build_tree_iter

"""
Test Cases
3
1 2 3 N N N N
2 N 1 N N 3 N N N N N N N
1 2 N 3 N N N 4 N N N N N N N N N

Output
1 2 3
2 1 3
1 2 3 4
"""
def level_order(root: TreeNode) -> list:
    if root is None:
        return []
    queue = [root]
    order_list = []

    while queue:
        node = queue.pop(0)
        order_list.append(node.data)

        if node.left:
            queue.append(node.left)
        
        if node.right:
            queue.append(node.right)
    
    return order_list


def main():
    test_cases: int = int(input())
    results = []
    for _ in range(test_cases):
        tree_str: str = input()
        root: TreeNode = build_tree_iter(tree_str)
        level_order_list: list = level_order(root)
        results.append(level_order_list)
    
    for res in results:
        print(" ".join(res))


if __name__ == "__main__":
    main()
