#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn add_node(&mut self, _a_node: ListNode) {
        todo!()
    }
    pub fn pop_node(&mut self) -> ListNode {
        todo!()
    }
    pub fn delete_node(&mut self) {
        todo!()
    }
    pub fn merge_list(&mut self, _other_list: &ListNode) {
        todo!()
    }
}
