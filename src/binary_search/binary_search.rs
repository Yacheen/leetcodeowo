pub fn search(nums: Vec<i32>, target: i32) -> i32 {
  let mut left_pointer: i32 = 0;
  let mut right_pointer: i32 = (nums.len() - 1) as i32;
  while left_pointer <= right_pointer {
      let middle: i32 = (left_pointer + right_pointer) / 2;
    
      if *nums.get(middle as usize).unwrap() > target {
          right_pointer = middle - 1;
      }
      else if *nums.get(middle as usize).unwrap() < target {
          left_pointer = middle + 1;
      }
      else {
          return middle;
      }
  }
  -1
}