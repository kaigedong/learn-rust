use std::cell::RefCell;
use std::rc::Rc;

// 定义一个二叉树
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 中序：左，中，右
        let mut root = root;
        let mut result = Vec::new();
        let mut stack = Vec::new();

        loop {
            if let Some(node) = root {
                root = node.borrow().left.clone();
                stack.push(node);
            } else if let Some(node) = stack.pop() {
                let node_ref = node.borrow();
                result.push(node_ref.val);

                root = node_ref.right.clone();
            } else {
                break;
            }
        }
        result
    }
}
