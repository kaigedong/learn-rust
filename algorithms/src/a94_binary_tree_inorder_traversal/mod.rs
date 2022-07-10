use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 中序：左，中，右
        let mut root = root;
        let mut result = Vec::new();
        let mut stack = Vec::new();

        loop {
            if let Some(node) = root {
                // 当前节点不为空时，将当前节点移到左节点，并将当前节点压栈
                // 因此，栈中存放的都是有值的节点
                root = node.borrow().left.clone();
                stack.push(node);
            } else if let Some(node) = stack.pop() {
                // 当前节点为空时，从栈中拿出来一个节点，保存其值，
                // 先拿出来的为左节点，
                // 下一次循环时，保存的值为左节点的根节点，即中间节点，
                // 并将当前节点移动到右边节点
                let node_ref = node.borrow();
                result.push(node_ref.val);

                // 并将当前节点指向右节点
                root = node_ref.right.clone();
            } else {
                break;
            }
        }
        result
    }
}
