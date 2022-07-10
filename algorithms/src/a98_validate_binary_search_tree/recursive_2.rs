use super::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution2;
impl Solution2 {
    #[allow(dead_code)]
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst_helper(root.as_deref(), i64::MAX, i64::MIN)
    }
    fn is_valid_bst_helper(root: Option<&RefCell<TreeNode>>, max: i64, min: i64) -> bool {
        if let Some(root) = root {
            let root = root.borrow();
            (root.val as i64) < max
                && (root.val as i64) > min
                && Self::is_valid_bst_helper(root.left.as_deref(), root.val as i64, min as i64)
                && Self::is_valid_bst_helper(root.right.as_deref(), max as i64, root.val as i64)
        } else {
            true
        }
    }
}
