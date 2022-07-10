use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut layer = VecDeque::new();
        layer.push_back(root.unwrap());

        let mut output = 0;
        loop {
            let layer_len = layer.len();
            if layer_len == 0 {
                break;
            }
            for i in 0..layer_len {
                let one_node = layer.pop_front().unwrap();
                if i == 0 {
                    output = one_node.borrow().val;
                }
                let one_node = one_node.borrow();
                let left = one_node.left.clone();
                let right = one_node.right.clone();
                match (left, right) {
                    (Some(left), Some(right)) => {
                        layer.push_back(left);
                        layer.push_back(right);
                    }
                    (Some(node), None) | (None, Some(node)) => layer.push_back(node),
                    _ => {}
                }
            }
        }

        output
    }
}
