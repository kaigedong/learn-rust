use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_unival_tree_helper(root.as_deref())
    }

    fn is_unival_tree_helper(root: Option<&RefCell<TreeNode>>) -> bool {
        if let Some(root) = root {
            let root = root.borrow();

            let left = root.left.as_deref();
            let right = root.right.as_deref();

            // 判断三个三个节点是相等的
            let is_equal_node = match (left, right) {
                (Some(left), Some(right)) => {
                    root.val == left.borrow().val && root.val == right.borrow().val
                }
                (Some(node), None) | (None, Some(node)) => root.val == node.borrow().val,
                (None, None) => true,
            };

            return is_equal_node
                && Self::is_unival_tree_helper(left)
                && Self::is_unival_tree_helper(right);
        }
        true
    }
}
