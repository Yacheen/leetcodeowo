pub fn is_anagram(s: String, t: String) -> bool {
  let mut s_chars: Vec<char> = s.chars().collect();
  let mut t_chars: Vec<char> = t.chars().collect();
  s_chars.sort_unstable();
  t_chars.sort_unstable();
  if s_chars.iter().collect::<String>().eq(&t_chars.iter().collect::<String>())  {
      return true;
  }
  else {
      return false;
  }
}