pub trait Puzzle {
  fn puzzle_a(&self, String) -> String;
  fn puzzle_b(&self, String) -> String;

  fn desc(&self) -> (String, String);
}
