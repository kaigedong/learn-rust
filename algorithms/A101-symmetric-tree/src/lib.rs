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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => {
                let node = node.borrow();
                Self::cmp(node.left.as_deref(), node.right.as_deref())
            }
        }
    }

    fn cmp(left: Option<&RefCell<TreeNode>>, right: Option<&RefCell<TreeNode>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                let left = left.borrow();
                let right = right.borrow();
                left.val == right.val
                    && Self::cmp(left.left.as_deref(), right.right.as_deref())
                    && Self::cmp(left.right.as_deref(), right.left.as_deref())
            }
            _ => false,
        }
    }
}
