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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root.as_deref();

        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        // p, q 大小不一定
        let val = if q_val > p_val {
            Self::lowest_common_ancestor_helper(root, p_val, q_val)
        } else {
            Self::lowest_common_ancestor_helper(root, q_val, p_val)
        };

        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    fn lowest_common_ancestor_helper(root: Option<&RefCell<TreeNode>>, p: i32, q: i32) -> i32 {
        if let Some(root) = root {
            let root = root.borrow();
            if root.val >= p && root.val <= q {
                root.val
            } else if root.val < p {
                return Self::lowest_common_ancestor_helper(root.right.as_deref(), p, q);
            } else {
                return Self::lowest_common_ancestor_helper(root.left.as_deref(), p, q);
            }
        } else {
            // 不会出现这种情况
            0
        }
    }
}
