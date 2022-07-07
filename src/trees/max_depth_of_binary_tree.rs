#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;
impl Solution {
    // recursive depth-first search
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth: i32 = 0;
        let mut max_left = 0;
        let mut max_right = 0;
        if let Some(node) = root.clone() {
            // get immutable version
            let node = node.borrow();
            
            1 as i32 + cmp::max(max_depth(node.left.clone()), max_depth(node.right.clone()))
            
        }
        else {
            return max_depth;
        }
    }
}