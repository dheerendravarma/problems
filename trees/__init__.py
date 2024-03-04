from typing import Optional


class TreeNode:
    def __init__(self, value) -> None:
        self.left = None
        self.right = None
        self.data = value

def buid_tree(tree_str: str) -> Optional[TreeNode]:
    # Return Empty Tree If no nodes found in the String
    nodes_length = len(tree_str)
    if nodes_length == 0 or tree_str[0] == "N":
        return
    
    input_nodes = list(map(str, tree_str.split()))

    # Create the QUEUE
    queue_length = 0
    queue = list()

    # create the root node
    root_node = TreeNode(input_nodes[0])
    queue.append(root_node)
    queue_length += 1

    idx = 1
    while(idx < nodes_length and queue_length > 0):
        node = queue.pop(0)
        queue_length -= 1

        left_value = input_nodes[idx]
        if left_value != "N":
            node.left = TreeNode(left_value)
            queue.append(node.left)
            queue_length += 1
        
        idx += 1
        if(idx >= nodes_length):
            break

        right_value = input_nodes[idx]
        if right_value != "N":
            node.right = TreeNode(right_value)
            queue.append(node.right)
            queue_length += 1
        idx += 1
    return root_node
