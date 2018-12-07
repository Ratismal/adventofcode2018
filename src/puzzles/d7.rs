use puzzles::puzzle::Puzzle;

use regex::Regex;
use std::collections::HashMap;

pub struct Day {}

impl Puzzle for Day {
  fn desc(&self) -> (String, String) {
    return (
      String::from("To be determined..."),
      String::from("To be determined..."),
    );
  }

  fn puzzle_a(&self, input: String) -> String {
    let map = self.parse(&input);
    let mut queue: Vec<String> = Vec::new();
    let mut completed: Vec<String> = Vec::new();
    for task in map.values() {
      if task.before.len() == 0 {
        queue.push(task.id.to_string());
      }
    }

    while queue.len() > 0 {
      let id = self.step(&map, &mut queue, &completed, true);
      completed.push(id);
    }
    return completed.join("");
  }

  fn puzzle_b(&self, input: String) -> String {
    let map = self.parse(&input);
    let mut queue: Vec<String> = Vec::new();
    let mut completed: Vec<String> = Vec::new();
    for task in map.values() {
      if task.before.len() == 0 {
        queue.push(task.id.to_string());
      }
    }
    let mut workers = 5;
    let mut seconds = 0;
    let mut in_progress: HashMap<String, i32> = HashMap::new();
    loop {
      for (t, start) in in_progress.clone() {
        if (start + self.time_required(&t)) <= seconds {
          in_progress.remove(&t);
          workers += 1;
          let task = map.get(&t).unwrap().clone();
          for tid in task.after {
            let task2: &Task = map.get(&tid).unwrap();
            let mut add = true;
            for bef in &task2.before {
              if bef != &t && !completed.contains(bef) {
                add = false;
              }
            }
            if add {
              queue.push(task2.id.to_string());
            }
          }
          completed.push(t.to_string());
          println!(
            "{:4}: A worker has finished {}.     There are {} workers remaining.",
            seconds, t, workers
          );
        }
      }

      while workers > 0 && queue.len() > 0 {
        let t = self.step(&map, &mut queue, &completed, false);
        in_progress.insert(t.to_string(), seconds);
        workers += -1;
        println!(
          "{:4}: A worker has started on {}.   There are {} workers remaining.",
          seconds, t, workers
        );
      }

      if queue.len() == 0 && workers == 5 {
        break;
      }
      seconds += 1;
    }
    return seconds.to_string();
  }
}

impl Day {
  fn parse(&self, input: &String) -> HashMap<String, Task> {
    lazy_static! {
      static ref line_re: Regex =
        Regex::new(r"^Step (\w) must be finished before step (\w) can begin\.$").unwrap();
    }
    let mut map: HashMap<String, Task> = HashMap::new();
    for line in input.lines() {
      for cap in line_re.captures_iter(line) {
        let id1 = cap[1].to_string();
        let id2 = cap[2].to_string();
        {
          let task1: &mut Task = map.entry(id1.to_string()).or_insert(Task {
            id: id1.to_string(),
            before: Vec::new(),
            after: Vec::new(),
          });
          task1.add_after(id2.to_string());
        }
        {
          let task2: &mut Task = map.entry(id2.to_string()).or_insert(Task {
            id: id2.to_string(),
            before: Vec::new(),
            after: Vec::new(),
          });
          task2.add_before(id1.to_string());
        }
      }
    }

    return map;
  }

  fn step(
    &self,
    map: &HashMap<String, Task>,
    queue: &mut Vec<String>,
    completed: &Vec<String>,
    populate: bool,
  ) -> String {
    // Sort in order of Z-A, so that A is at the end (and poppable)
    queue.sort_by(|a, b| b.cmp(a));
    let id = queue.pop().unwrap();
    let task = map.get(&id).unwrap().clone();
    if populate {
      for tid in task.after {
        let t: &Task = map.get(&tid).unwrap();
        let mut add = true;
        for bef in &t.before {
          if bef != &id && !completed.contains(bef) {
            add = false;
          }
        }
        if add {
          queue.push(t.id.to_string());
        }
      }
    }

    return id;
  }

  fn time_required(&self, id: &String) -> i32 {
    let mut i = 0;
    for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
      i += 1;
      if c.to_string() == *id {
        break;
      }
    }
    return 60 + i;
  }
}

struct Task {
  id: String,
  before: Vec<String>,
  after: Vec<String>,
}

impl Task {
  fn add_before(&mut self, input: String) {
    let mut before = self.before.clone();
    before.push(input);
    self.before = before;
  }

  fn add_after(&mut self, input: String) {
    let mut after = self.after.clone();
    after.push(input);
    self.after = after;
  }

  fn clone(&self) -> Task {
    Task {
      id: self.id.to_string(),
      before: self.before.clone(),
      after: self.after.clone(),
    }
  }
}
