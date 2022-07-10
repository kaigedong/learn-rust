use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        root.map_or(true, |root| {
            Self::is_symmetric_helper(
                root.borrow().left.as_deref(),
                root.borrow().right.as_deref(),
            )
        })
    }
    fn is_symmetric_helper(
        left: Option<&RefCell<TreeNode>>,
        right: Option<&RefCell<TreeNode>>,
    ) -> bool {
        match (left, right) {
            (Some(left), Some(right)) => {
                let left = left.borrow();
                let right = right.borrow();
                left.val == right.val
                    && Self::is_symmetric_helper(left.left.as_deref(), right.right.as_deref())
                    && Self::is_symmetric_helper(left.right.as_deref(), right.left.as_deref())
            }
            (None, None) => true,
            _ => false,
        }
    }
}
