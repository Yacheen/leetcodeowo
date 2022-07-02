use std::collections::HashMap;
pub fn is_valid(s: String) -> bool {
  // i think because this is a stack question, i need to have a "stack"
  let mut current_stack = Vec::new();
  let brackets: HashMap<char, char> = HashMap::from([
    ('}', '{'),
    (')', '('),
    (']', '['),
  ]);
  let s = s.chars().collect::<Vec<_>>();
  for letter in s.iter() {
    if let Some((closing_bracket, opening_bracket)) = brackets.get_key_value(letter) {
      if let Some(current_bracket) = current_stack.last() {
        if *current_bracket == opening_bracket {
          current_stack.pop();
        }
        else {
          return false;
        }
      }
      else {
        return false;
      }
    }

    else {
      current_stack.push(letter);
    }
  }
  if current_stack.len() > 0 {return false;} else {return true;}
}