use std::collections::HashMap;

use puzzles::puzzle::Puzzle;

pub struct Day {}

impl Puzzle for Day {
  fn desc(&self) -> (String, String) {
    return (
      String::from("Finds a checksum from IDs"),
      String::from("Find IDs one step apart"),
    );
  }

  fn puzzle_a(&self, input: String) -> String {
    let lines = input.lines();
    let mut two_count = 0;
    let mut three_count = 0;
    for line in lines {
      let (two, three) = self.analyze(String::from(line));
      if two {
        two_count += 1;
      }
      if three {
        three_count += 1;
      }
    }

    return (two_count * three_count).to_string();
  }

  fn puzzle_b(&self, input: String) -> String {
    let mut i: u32 = 0;
    for line_a in input.lines() {
      let mut j: u32 = 0;
      for line_b in input.lines() {
        if j < i {
          j += 1;
          continue;
        }
        let (res, matches) = self.compare(line_a, line_b);
        if res {
          return matches;
        }
        j += 1;
      }
      i += 1;
    }

    return String::from("No match found");
  }
}

impl Day {
  fn analyze(&self, line: String) -> (bool, bool) {
    let mut map: HashMap<char, u32> = HashMap::new();
    let mut two = false;
    let mut three = false;

    let char_vec: Vec<char> = line.chars().collect();
    for c in char_vec {
      let count = map.entry(c).or_insert(0);
      *count += 1;
    }

    for (_, val) in &map {
      if val == &2u32 {
        two = true;
      }
      if val == &3u32 {
        three = true;
      }
    }

    return (two, three);
  }
  fn compare(&self, a: &str, b: &str) -> (bool, String) {
    let a_char: Vec<char> = a.chars().collect();
    let b_char: Vec<char> = b.chars().collect();

    let mut matches = String::from("");
    let mut misses = 0;

    for i in 0..a_char.len() {
      if a_char[i] == b_char[i] {
        matches = format!("{}{}", matches, a_char[i].to_string());
      } else {
        misses += 1;
      }
    }

    return (misses == 1, matches);
  }
}
