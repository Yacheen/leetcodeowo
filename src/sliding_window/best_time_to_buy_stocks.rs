use std::cmp;
pub fn max_profit(prices: Vec<i32>) -> i32 {
  let mut left = 0;
  let mut right = 1;
  let mut max_profit = 0;

  while right < prices.len() {
      if prices.get(left).unwrap() < prices.get(right).unwrap() {
          let current_profit = prices.get(right).unwrap() - prices.get(left).unwrap();
          max_profit = cmp::max(max_profit, current_profit);
      }
      else {
          left = right;
      }
      right += 1;
  }

  max_profit as i32
}