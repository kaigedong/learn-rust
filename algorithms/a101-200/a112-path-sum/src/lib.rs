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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::has_path_sum_helper(root.as_deref(), 0, target_sum)
    }

    // 必须到叶子节点才计算和
    fn has_path_sum_helper(
        root: Option<&RefCell<TreeNode>>,
        sum_before: i32,
        target_sum: i32,
    ) -> bool {
        root.map_or(false, |root| {
            let root = root.borrow();
            let val = root.val;
            let current_sum = val + sum_before;

            let left = root.left.as_deref();
            let right = root.right.as_deref();

            if left.is_none() && right.is_none() && current_sum == target_sum {
                return true;
            }

            Self::has_path_sum_helper(root.left.as_deref(), current_sum, target_sum)
                || Self::has_path_sum_helper(root.right.as_deref(), current_sum, target_sum)
        })
    }
}
