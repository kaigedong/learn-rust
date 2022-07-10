use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::bst(&nums)
    }

    fn bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            None
        } else {
            let mid = nums.len() / 2;
            // 以某个index进行分割
            let (left, rest) = nums.split_at(mid);
            // 返回第一个值和剩下的slice
            let (&val, right) = rest.split_first().unwrap();

            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: Self::bst(left),
                right: Self::bst(right),
            })))
        }
    }
}
