// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

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
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root.as_deref() {
            let root = root.borrow();
            let left = root.left.as_deref();
            let right = root.right.as_deref();
            if let (Some(left), Some(right)) = (left, right) {
                if root.val == left.borrow().val + right.borrow().val {
                    return true;
                }
            }
        }
        false
    }
}
