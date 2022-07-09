use std::cell::RefCell;
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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut out = vec![];

        root.map_or((), |root| {
            let root = root.borrow();
            let prefix = format!("{}", root.val);
            if root.left.is_none() && root.right.is_none() {
                out.push(prefix);
            } else {
                Self::binary_tree_paths_helper(root.left.as_deref(), prefix.clone(), &mut out);
                Self::binary_tree_paths_helper(root.right.as_deref(), prefix, &mut out);
            }
        });
        out
    }
    fn binary_tree_paths_helper(
        root: Option<&RefCell<TreeNode>>,
        prefix: String,
        out: &mut Vec<String>,
    ) {
        if let Some(root) = root {
            let root = root.borrow();
            let prefix = format!("{}->{}", prefix, root.val);

            let left = root.left.as_deref();
            let right = root.right.as_deref();

            match (left, right) {
                (Some(_), Some(_)) => {
                    Self::binary_tree_paths_helper(left, prefix.clone(), out);
                    Self::binary_tree_paths_helper(right, prefix, out);
                }
                (Some(node), None) | (None, Some(node)) => {
                    Self::binary_tree_paths_helper(Some(node), prefix, out);
                }
                (None, None) => {
                    out.push(prefix);
                }
            }
        } else {
            out.push(prefix);
        }
    }
}
