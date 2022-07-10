use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut out = vec![];
        let mut layer = VecDeque::new();
        if let Some(root) = root {
            let root = root;
            layer.push_back(root);
        }

        loop {
            let layer_len = layer.len();
            if layer_len == 0 {
                break;
            }
            for node_index in 0..layer_len {
                // let node = layer[node_index].clone();
                let node = layer.pop_front().unwrap();
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();

                if node_index == layer_len - 1 {
                    out.push(node.borrow().val);
                }

                match (left, right) {
                    (Some(left), Some(right)) => {
                        layer.push_back(left);
                        layer.push_back(right);
                    }
                    (Some(node), None) | (None, Some(node)) => {
                        layer.push_back(node);
                    }
                    _ => {}
                }
            }
        }
        out
    }
}
