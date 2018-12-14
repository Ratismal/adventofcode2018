use std::collections::HashMap;

use puzzles::puzzle::Puzzle;

pub struct Day {}

struct Node {
  meta: Vec<u32>,
}

enum State {
  HEADER_CHILD,
  HEADER_META,
  META,
}

impl Puzzle for Day {
  fn desc(&self) -> (String, String) {
    return (String::from("Placeholder"), String::from("Placeholder"));
  }

  fn puzzle_a(&self, input: String) -> String {
    let elements = self.parse(&input);

    let mut state: State = State::HEADER_CHILD;
    let mut child_nodes: u32;
    let mut meta_elements: u32;
    for i in elements {}

    return String::from("Placeholder");
  }

  fn puzzle_b(&self, input: String) -> String {
    let elements = self.parse(&input);

    return String::from("Placeholder");
  }
}

impl Day {
  fn parse(&self, input: &String) -> Vec<u32> {
    let mut elements: Vec<u32> = Vec::new();
    for set in input.split(" ") {
      let i: u32 = set.parse().unwrap();
      elements.push(i);
    }
    return elements;
  }
}
