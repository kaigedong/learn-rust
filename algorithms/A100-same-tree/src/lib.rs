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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // as_deref:
        // Converts from `Option<T>` (or `&Option<T>`) to `Option<&T::Target>`.
        Self::is_same_tree_helper(p.as_deref(), q.as_deref())
    }

    fn is_same_tree_helper(p: Option<&RefCell<TreeNode>>, q: Option<&RefCell<TreeNode>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let p = p.borrow(); // TreeNode类型
                let q = q.borrow();

                p.val == q.val
                    && Self::is_same_tree_helper(p.left.as_deref(), q.left.as_deref())
                    && Self::is_same_tree_helper(p.right.as_deref(), q.right.as_deref())
            }
            _ => false,
        }
    }
}
