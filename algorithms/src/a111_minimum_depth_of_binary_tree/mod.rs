use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::min_depth_helper(root.as_deref())
    }

    // 计算子树深度
    // 注意，左右节点均为空时，才算叶子节点，这时候返回
    fn min_depth_helper(root: Option<&RefCell<TreeNode>>) -> i32 {
        root.map_or(0, |root| {
            let root = root.borrow();
            let left = root.left.as_deref();
            let right = root.right.as_deref();

            match (left, right) {
                (None, None) => 1,
                _ => {
                    let left_depth = Self::min_depth_helper(left);
                    let right_depth = Self::min_depth_helper(right);

                    if left_depth == 0 {
                        return right_depth + 1;
                    }
                    if right_depth == 0 {
                        return left_depth + 1;
                    }
                    i32::min(left_depth + 1, right_depth + 1)
                }
            }
        })
    }
}
