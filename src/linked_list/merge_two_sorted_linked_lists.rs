// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // fix
    let mut dummy = Some(Box::new(ListNode::new(0)));
    let mut tail = dummy.as_mut();

    loop {
      if let Some(list1_node) = list1.as_mut() {
        if let Some(list2_node) = list2.as_mut() {
          if list1_node.val < list2_node.val {
            tail.as_mut().unwrap().next = Some(list1_node.clone()); 
            list1 = list1.unwrap().next;
          }
          else {
            tail.as_mut().unwrap().next = Some(list2_node.clone());
            list2 = list2.unwrap().next;
          }
          tail = tail.unwrap().next.as_mut();
        }
        else {
          break;
        }
      } 
      else {
        break;
      }
    }

    // something ran out, add to list
    if let Some(_list1_node) = list1.as_ref() {
      tail.as_mut().unwrap().next = Some(list1.unwrap().clone());
    }
    else if let Some(_list2_node) = list2.as_ref() {
      tail.as_mut().unwrap().next = Some(list2.unwrap().clone());
    }
    dummy.unwrap().next
}