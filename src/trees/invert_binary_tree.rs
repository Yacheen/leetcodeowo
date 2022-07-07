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
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
  if let Some(node) = root.clone(){ 
    println!("rc is currently {:?}", Rc::strong_count(&node));
    let mut node = node.borrow_mut();
    // let tmp = node.left.clone();
    node.left = node.right.clone();
    node.right = node.left.clone();
    
    invert_tree(node.left.clone());
    invert_tree(node.right.clone());
    
    return root;
  }
  else {
    return None;
  }
}