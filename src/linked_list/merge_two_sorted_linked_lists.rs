//what i got so far
pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  // fix
let mut dummy = Some(Box::new(ListNode::new(0)));
  let mut tail = dummy;
  while let (Some(list1_node), Some(list2_node)) = (list1.clone(), list2.clone()) {
    if list1_node.val < list2_node.val {
      tail.as_mut().unwrap().next = Some(list1_node.clone());
      list1 = list1_node.next;
    }
    else {
      tail.as_mut().unwrap().next = Some(list2_node.clone());
      list2 = list2_node.next;
    }
  }
  tail
}