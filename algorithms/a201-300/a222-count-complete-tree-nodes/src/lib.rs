// Definition for a binary tree node.
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
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = root.as_deref();
        Self::count_nodes_helper(root)
    }
    fn count_nodes_helper(root: Option<&RefCell<TreeNode>>) -> i32 {
        if let Some(root) = root {
            let root = root.borrow();
            let left = root.left.as_deref();
            let right = root.right.as_deref();

            1 + Self::count_nodes_helper(left) + Self::count_nodes_helper(right)
        } else {
            0
        }
    }
}
