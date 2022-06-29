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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_balanced_helper(root.as_deref())
    }

    fn is_balanced_helper(root: Option<&RefCell<TreeNode>>) -> bool {
        match root {
            None => true,
            Some(node) => {
                let node = node.borrow();
                let left = node.left.as_deref();
                let right = node.right.as_deref();

                let l_height = Self::tree_depth(left);
                let r_height = Self::tree_depth(right);
                if i32::abs(l_height - r_height) > 1 {
                    return false;
                }
                Self::is_balanced_helper(left) && Self::is_balanced_helper(right)
            }
        }
    }

    fn tree_depth(root: Option<&RefCell<TreeNode>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                // NOTE: 重要的一行，防止临时变量在行末尾
                let node = node.borrow();
                let left = node.left.as_deref();
                let right = node.right.as_deref();
                i32::max(Self::tree_depth(left), Self::tree_depth(right)) + 1
            }
        }
    }
}
