use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub type NodeRef = Rc<RefCell<TreeNode>>;
pub type TreeLink = Option<NodeRef>;

#[derive(Debug)]
pub struct TreeNode {
    pub data: i64,
    pub left: TreeLink,
    pub right: TreeLink,
}

impl TreeNode {
    pub fn new(data: i64) -> NodeRef {
        Rc::new(RefCell::new(TreeNode {
            data,
            left: None,
            right: None,
        }))
    }
}

/// BFS-based tree construction matching Python's build_tree() in trees/__init__.py.
/// Input: space-separated token string; "N" = null node.
pub fn build_tree(tree_str: &str) -> TreeLink {
    let tokens: Vec<&str> = tree_str.split_whitespace().collect();
    if tokens.is_empty() || tokens[0] == "N" {
        return None;
    }

    let root_val: i64 = tokens[0].parse().expect("invalid node value");
    let root = TreeNode::new(root_val);
    let mut queue: VecDeque<NodeRef> = VecDeque::new();
    queue.push_back(root.clone());

    let mut idx = 1usize;
    let n = tokens.len();

    while !queue.is_empty() && idx < n {
        let node = queue.pop_front().unwrap();

        if tokens[idx] != "N" {
            let val: i64 = tokens[idx].parse().expect("invalid node value");
            let left_child = TreeNode::new(val);
            node.borrow_mut().left = Some(left_child.clone());
            queue.push_back(left_child);
        }
        idx += 1;
        if idx >= n {
            break;
        }

        if tokens[idx] != "N" {
            let val: i64 = tokens[idx].parse().expect("invalid node value");
            let right_child = TreeNode::new(val);
            node.borrow_mut().right = Some(right_child.clone());
            queue.push_back(right_child);
        }
        idx += 1;
    }

    Some(root)
}
