use puzzles::puzzle::Puzzle;

pub struct Day {}

impl Puzzle for Day {
  fn desc(&self) -> (String, String) {
    return (
      String::from("To be determined..."),
      String::from("To be determined..."),
    );
  }

  fn puzzle_a(&self, input: String) -> String {
    let _lines = input.lines();
    return String::from("Placeholder");
  }

  fn puzzle_b(&self, input: String) -> String {
    let _lines = input.lines();
    return String::from("Placeholder");
  }
}
