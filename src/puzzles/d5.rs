use puzzles::puzzle::Puzzle;

pub struct Day {}

impl Puzzle for Day {
  fn desc(&self) -> (String, String) {
    return (
      String::from("Eliminate differing cases"),
      String::from("To be determined..."),
    );
  }

  fn puzzle_a(&self, input: String) -> String {
    return self.react(input).to_string();
  }

  fn puzzle_b(&self, input: String) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyz".chars();
    let mut lengths: Vec<usize> = Vec::new();
    for c in chars {
      let replaced = self.replace(input.clone(), c);
      lengths.push(self.react(replaced));
    }
    lengths.sort_by(|a, b| a.cmp(b));
    return lengths[0].to_string();
  }
}

impl Day {
  fn react(&self, input: String) -> usize {
    let mut chars: Vec<char> = input.chars().into_iter().collect();
    let mut last: char = '0';
    let mut changes: bool = true;
    while changes {
      changes = false;
      for i in (0..(chars.len() - 1)).rev() {
        if i < chars.len() {
          let c: char = chars[i];
          if c.to_lowercase().to_string() == last.to_lowercase().to_string() && c != last {
            chars.remove(i + 1);
            chars.remove(i);
            changes = true
          }
        }
        last = chars[i];
      }
    }
    return chars.len();
  }

  fn replace(&self, input: String, rep: char) -> String {
    let lower: Vec<char> = rep.to_lowercase().collect();
    let upper: Vec<char> = rep.to_uppercase().collect();
    return input.replace(lower[0], "").replace(upper[0], "");
  }
}
