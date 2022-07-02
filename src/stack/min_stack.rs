struct MinStack (Vec<i32>);
/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
      MinStack(Vec::new())
    }
    
    fn push(&mut self, val: i32) {
        self.0.push(val);
    }
    
    fn pop(&mut self) {
        self.0.pop();
    }
    
    fn top(&self) -> i32 {
        match self.0.last() {
          Some(val) => *val,
          None => panic!()
        }
    }
    
    fn get_min(&self) -> i32 {
        if let Some(val) = self.0.get(0) {
          let mut lowest: i32 = *val;
          for number in self.0.iter() {
            if *number < lowest {
              lowest = *number;
            }
            else {
              continue;
            }
          }
          lowest
        } 
        else {
          panic!();
        }
    }
}
