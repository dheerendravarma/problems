from typing import Optional


class TreeNode:
    def __init__(self, value) -> None:
        self.left = None
        self.right = None
        self.data = value

def buid_tree(tree_str: str) -> Optional[TreeNode]:
    # Return Empty Tree If no nodes found in the String
    if len(tree_str) == 0 or tree_str[0] == "N":
        return
    
    input_nodes = list(map(str, tree_str.split()))
    nodes_length = len(input_nodes)

    def build_node(index):
        if index < nodes_length and input_nodes[index] != 'N':
            root = TreeNode(input_nodes[index])
            root.left = build_node(2 * index + 1)
            root.right = build_node(2 * index + 2)
            return root
        return None

    return build_node(0)
