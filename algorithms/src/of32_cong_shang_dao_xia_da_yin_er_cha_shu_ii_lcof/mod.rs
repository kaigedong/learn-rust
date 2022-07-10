use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut out = vec![];
        let mut layer = VecDeque::new();

        if let Some(root) = root {
            layer.push_back(root);
        }

        while !layer.is_empty() {
            let mut layer_out = vec![];
            let layer_len = layer.len();
            for _ in 0..layer_len {
                let node = layer.pop_front().unwrap();
                let node = node.borrow();
                layer_out.push(node.val);
                if node.left.is_some() {
                    layer.push_back(node.left.clone().unwrap());
                }
                if node.right.is_some() {
                    layer.push_back(node.right.clone().unwrap());
                }
            }
            out.push(layer_out);
        }
        out
    }
}
