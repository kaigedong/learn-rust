use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = root.as_deref();
        let (diameter, _depth) = Self::get_depth(root);
        diameter
    }

    // 二叉树的 每个节点的左右子树的高度和 的最大值。
    // 返回两个：最大直径和当前节点的深度
    fn get_depth(root: Option<&RefCell<TreeNode>>) -> (i32, i32) {
        root.map_or((0, 0), |root| {
            let root = root.borrow();
            let left = root.left.as_deref();
            let right = root.right.as_deref();

            let (l_diameter, l_depth) = Self::get_depth(left);
            let (r_diameter, r_depth) = Self::get_depth(right);

            (
                (l_diameter).max(r_diameter).max(l_depth + r_depth),
                l_depth.max(r_depth) + 1,
            )
        })
    }
}
