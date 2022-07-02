use std::collections::HashSet;
// using a hashmap: time complexity: 16ms, space complexity: 4.5mb
// using 2 for loops: O(n^2): time complexity: 598ms, space complexity: 3.1mb
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut distinct_nums = HashSet::new();
    
    for number in nums.iter() {
        if distinct_nums.contains_key(number) {
            return true;
        }
        else {
            distinct_nums.insert(number, true);
            continue;
        }
    }
    return false;
}
//input:
// let some_bool = contains_duplicate(Vec::from([1, 2, 3,]));
// println!("{}", some_bool);
