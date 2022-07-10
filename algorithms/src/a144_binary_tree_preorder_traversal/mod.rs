use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 前序遍历
        let mut out = vec![];
        let root = root.as_deref();
        Self::preorder_traversal_helper(root, &mut out);
        out
    }

    fn preorder_traversal_helper(root: Option<&RefCell<TreeNode>>, out: &mut Vec<i32>) {
        if let Some(root) = root {
            out.push(root.borrow().val);
            Self::preorder_traversal_helper(root.borrow().left.as_deref(), out);
            Self::preorder_traversal_helper(root.borrow().right.as_deref(), out);
        }
    }
}
