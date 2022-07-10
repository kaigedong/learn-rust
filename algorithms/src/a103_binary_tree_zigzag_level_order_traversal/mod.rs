use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut out = vec![];
        let mut layer = VecDeque::new();
        if let Some(root) = root {
            layer.push_back(root);
        }

        let mut left_to_right = true;
        while !layer.is_empty() {
            let layer_len = layer.len();
            let mut out_layer = vec![];

            for _ in 0..layer_len {
                let node = layer.pop_front().unwrap();
                let node = node.borrow();
                if let Some(left) = node.left.clone() {
                    layer.push_back(left);
                }
                if let Some(right) = node.right.clone() {
                    layer.push_back(right);
                }
                out_layer.push(node.val);
            }

            if left_to_right {
                out.push(out_layer);
            } else {
                out.push(out_layer.into_iter().rev().collect());
            }

            left_to_right = !left_to_right;
        }

        out
    }
}
