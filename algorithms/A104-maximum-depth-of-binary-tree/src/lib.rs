use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // TODO: 1. 深度优先
        // TODO: 2. 广度优先
        // 3. 递归

        Self::max_depth_helper(root.as_deref())
    }

    fn max_depth_helper(root: Option<&RefCell<TreeNode>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                i32::max(
                    Self::max_depth_helper(node.left.as_deref()),
                    Self::max_depth_helper(node.right.as_deref()),
                ) + 1
            }
        }
    }
}
