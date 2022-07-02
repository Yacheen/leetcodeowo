pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  
  // let mut my_map = HashMap::new();
  // let mut result: Vec<i32> = Vec::new();
  // for (index, number) in nums.iter().enumerate() {
  //     my_map.insert(index, number);
  //     for (key, value) in my_map.iter() {
  //         if *key == index {
  //             continue;
  //         }
  //         else if number + *value == target {
  //             result.push(index as i32);
  //             result.push(*key as i32);
  //         }
  //         else {
  //             continue;
  //         }
  //     }

  // }
  // return result;

  let mut result: Vec<i32> = Vec::new();
  for (index, number) in nums.iter().enumerate() {
      for (other_indexes, other_numbers) in nums.iter().enumerate() {
          if other_indexes == index {
              continue;
          }
          else {
              if other_numbers + number == target {
                  result.push(index as i32);
                  result.push(other_indexes as i32);
                  return result;
              }
          }
      }
  }
  return result;
}