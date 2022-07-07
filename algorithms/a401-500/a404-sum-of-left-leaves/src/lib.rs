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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_of_left_leaves_helper(root.as_deref())
    }

    pub fn sum_of_left_leaves_helper(root: Option<&RefCell<TreeNode>>) -> i32 {
        let mut sum = 0;
        if let Some(root) = root {
            let root = root.borrow();
            let left = root.left.as_deref();
            let right = root.right.as_deref();

            // 如果是左叶子节点，sum加上该值
            if let Some(left) = left {
                let node = left.borrow();
                let node_left = node.left.as_deref();
                let node_right = node.right.as_deref();
                if let (None, None) = (node_left, node_right) {
                    sum += node.val;
                }
            }
            sum += Self::sum_of_left_leaves_helper(left);
            sum += Self::sum_of_left_leaves_helper(right);
        }
        sum
    }
}
