from typing import Optional


class TreeNode:
    def __init__(self, value) -> None:
        self.left = None
        self.right = None
        self.data = value

def build_tree_rec(tree_str: str) -> Optional[TreeNode]:
    # Return Empty Tree If no nodes found in the String
    # This Tree Build code is based on 2 * n + 1 and 2*n+2
    # Parent-Child Notation
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


def build_tree_iter(tree_str: str) -> Optional[TreeNode]:
    # Return Empty Tree If no nodes found in the String
    # This Tree Build code is based on 2 * n + 1 and 2*n+2
    # Parent-Child Notation
    if len(tree_str) == 0 or tree_str[0] == "N":
        return
    
    input_nodes = list(map(str, tree_str.split()))
    nodes_length = len(input_nodes)

    root_node = TreeNode(input_nodes[0])
    queue = [(0, root_node)]

    idx = 1
    while idx < nodes_length and queue:
        node_idx, node = queue.pop(0)

        left_child_idx = 2 * node_idx + 1
        if left_child_idx < nodes_length and input_nodes[left_child_idx] != "N":
            left_node = TreeNode(input_nodes[left_child_idx])
            node.left = left_node
            queue.append((left_child_idx, left_node))
        
        right_child_idx = 2 * node_idx + 2
        if right_child_idx < nodes_length and input_nodes[right_child_idx] != "N":
            right_node = TreeNode(input_nodes[right_child_idx])
            node.right = right_node
            queue.append((right_child_idx, right_node))
        
        idx += 1
    return root_node

def build_tree(tree_str: str) -> Optional[TreeNode]:
    if len(tree_str) == 0 or tree_str[0] == "N":
        return
    
    input_nodes = list(map(str, tree_str.split()))
    nodes_length = len(input_nodes)

    root_node = TreeNode(input_nodes[0])
    queue = [root_node]
    size = 1
    idx = 1

    while size > 0 and idx < nodes_length:
        node = queue.pop(0)
        size -= 1

        if input_nodes[idx] != "N":
            node.left = TreeNode(input_nodes[idx])
            queue.append(node.left)
            size += 1
        
        idx += 1
        if idx >= nodes_length:
            break

        if input_nodes[idx] != "N":
            node.right = TreeNode(input_nodes[idx])
            queue.append(node.right)
            size += 1
        
        idx += 1
    return root_node
