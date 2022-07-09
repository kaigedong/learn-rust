use std::cell::RefCell;
use std::collections::VecDeque;
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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut out = vec![];
        let mut layer = VecDeque::new();
        if let Some(root) = root {
            layer.push_back(root);
        }

        while !layer.is_empty() {
            let mut sum = 0f64;
            let layer_len = layer.len();
            for _ in 0..layer_len {
                let node = layer.pop_front().unwrap();
                let node = node.borrow();
                sum += node.val as f64;
                if let Some(left) = node.left.clone() {
                    layer.push_back(left);
                }
                if let Some(right) = node.right.clone() {
                    layer.push_back(right);
                }
            }
            out.push(sum / layer_len as f64);
        }
        out
    }
}
