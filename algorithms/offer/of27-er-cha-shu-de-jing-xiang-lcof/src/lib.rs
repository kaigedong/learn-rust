use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
struct Solution;

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
impl Solution {
    #[allow(dead_code)]
    pub fn mirror_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root_ref = root.as_deref();
        Self::mirror_tree_helper(root_ref);
        root
    }
    fn mirror_tree_helper(root: Option<&RefCell<TreeNode>>) {
        if let Some(root) = root {
            // 现在root类型是&mut TreeNode
            // 不加后面的*，是&mut RefMut<TreeNode>, borrow_mut会返回RefMut类型
            let root = &mut *root.borrow_mut();
            mem::swap(&mut root.left, &mut root.right);
            Self::mirror_tree_helper(root.left.as_deref());
            Self::mirror_tree_helper(root.right.as_deref());
        }
    }
}
