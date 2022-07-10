use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut out = vec![];

        let root = root.as_deref();
        Self::level_order_bottom_helper(root, 0, &mut out);

        out.into_iter().rev().collect()
    }

    fn level_order_bottom_helper(
        root: Option<&RefCell<TreeNode>>,
        level: usize,
        out: &mut Vec<Vec<i32>>,
    ) {
        if let Some(root) = root {
            let root = root.borrow();
            if out.len() < level + 1 {
                out.push(vec![])
            }
            out[level].push(root.val);

            let left = root.left.as_deref();
            let right = root.right.as_deref();
            Self::level_order_bottom_helper(left, level + 1, out);
            Self::level_order_bottom_helper(right, level + 1, out);
        }
    }
}
