use crate::data_structures::ListNode;
use std::mem;

pub struct Solution;
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (list1, list2);
        let mut result = None;
        // target 为 result 的可变引用
        let mut target = &mut result;

        loop {
            match (&mut l1, &mut l2) {
                (None, None) => break,
                (None, rest @ Some(_)) | (rest @ Some(_), None) => {
                    *target = rest.take();
                    break;
                }
                (Some(left), Some(right)) => {
                    *target = if left.val <= right.val {
                        let new_l1 = left.next.take();
                        mem::replace(&mut l1, new_l1)
                    } else {
                        let new_l2 = right.next.take();
                        mem::replace(&mut l2, new_l2)
                    };
                    target = &mut target.as_mut().unwrap().next;
                }
            }
        }

        result
    }
}
