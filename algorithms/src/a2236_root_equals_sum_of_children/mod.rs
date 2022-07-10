// Definition for a binary tree node.
use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

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
