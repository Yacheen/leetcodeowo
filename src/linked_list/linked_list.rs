// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}
// 
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None; // nothing
    let mut current = head; // (1, box<listnode>)
    while let Some(mut list_node) = current {
      let nxt = list_node.next; // this is 2, hold temporarily 
      list_node.next = prev; // set next to prev (if its 1, it'd be None)
      prev = Some(list_node); // set prev to what next val would need, which is currently 1
      current = nxt; // set current to temporarily held next pointer, which is 2
    }
    head = prev;
    head
    
  }