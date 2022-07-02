pub fn is_palindrome(mut s: String) -> bool {
  s = s.to_lowercase();
  s.retain(|letter| !letter.is_whitespace() && letter.is_alphanumeric());
  let mut s_as_vec = s.chars().collect::<Vec<char>>();
  let unreversed_string = s_as_vec.iter().collect::<String>();
  
  s_as_vec.reverse();
  let result = s_as_vec.iter().collect::<String>();
  if result == unreversed_string {
      return true;
  }
  else {
      return false;
  }
}