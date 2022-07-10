use crate::data_structures::TreeNode;
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
