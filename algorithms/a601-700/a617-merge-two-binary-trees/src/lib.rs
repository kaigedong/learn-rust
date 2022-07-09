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

use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::merge_trees_helper(root1.as_deref(), root2.as_deref())
    }
    fn merge_trees_helper(
        root1: Option<&RefCell<TreeNode>>,
        root2: Option<&RefCell<TreeNode>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(root1), Some(root2)) => {
                let root1 = root1.borrow();
                let root2 = root2.borrow();
                Some(Rc::new(RefCell::new(TreeNode {
                    val: root1.val + root2.val,
                    left: Self::merge_trees_helper(root1.left.as_deref(), root2.left.as_deref()),
                    right: Self::merge_trees_helper(root1.right.as_deref(), root2.right.as_deref()),
                })))
            }
            (Some(node), None) => {
                let node = node.borrow();
                Some(Rc::new(RefCell::new(TreeNode {
                    val: node.val,
                    left: Self::merge_trees_helper(node.left.as_deref(), None),
                    right: Self::merge_trees_helper(node.right.as_deref(), None),
                })))
            }
            (None, Some(node)) => {
                let node = node.borrow();
                Some(Rc::new(RefCell::new(TreeNode {
                    val: node.val,
                    left: Self::merge_trees_helper(None, node.left.as_deref()),
                    right: Self::merge_trees_helper(None, node.right.as_deref()),
                })))
            }
            _ => None,
        }
    }
}
