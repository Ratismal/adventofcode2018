use puzzles::puzzle::Puzzle;
use std::collections::HashMap;

// time to bite the bullet and use regex!
use regex::Regex;

enum LogEntryType {
  SHIFT,
  SLEEP,
  WAKE,
}

struct LogEntry {
  entry_type: LogEntryType,
  date: String,
  hour: u32,
  minute: u32,
  id: u32,
}

impl LogEntry {
  fn serialize(&self) -> String {
    return format!("{}_{:02}:{:02}", self.date, self.hour, self.minute);
  }
}

struct Guard {
  minutes_asleep: u32,
  minute_map: HashMap<u32, u32>,
  id: u32,
}

impl Guard {
  fn frequent_minute(&self) -> u32 {
    let mut highest_count: u32 = 0;
    let mut highest_minute: u32 = 0;
    for (minute, count) in &self.minute_map {
      if count > &highest_count {
        highest_count = *count;
        highest_minute = *minute;
      } else if count == &highest_count && minute < &highest_minute {
        highest_minute = *minute;
      }
    }
    return highest_minute;
  }
  fn get_count(&self, minute: u32) -> u32 {
    if self.minute_map.contains_key(&minute) {
      let count = self.minute_map.get(&minute).unwrap();
      return *count;
    } else {
      return 0;
    }
  }
}

pub struct Day {}

impl Puzzle for Day {
  fn desc(&self) -> (String, String) {
    return (String::from("Placeholder"), String::from("Placeholder"));
  }

  fn puzzle_a(&self, input: String) -> String {
    let guard_map = self.parse_guards(input);

    let mut guards: Vec<Guard> = Vec::new();

    for (_, guard) in guard_map {
      guards.push(guard);
    }

    guards.sort_by(|a, b| b.minutes_asleep.cmp(&a.minutes_asleep));
    let sleepiest = &guards[0];
    let highest_minute = sleepiest.frequent_minute();

    println!("ID: {}, Minute: {}", sleepiest.id, highest_minute);

    return (sleepiest.id * highest_minute).to_string();
  }

  fn puzzle_b(&self, input: String) -> String {
    let guard_map = self.parse_guards(input);

    let mut highest_count: u32 = 0;
    let mut highest_minute: u32 = 0;
    let mut highest_guard = Guard {
      id: 0,
      minutes_asleep: 0,
      minute_map: HashMap::new(),
    };
    for (_, guard) in guard_map {
      let minute = guard.frequent_minute();
      let count = &guard.get_count(minute);

      println!("#{}: {} ({})", guard.id, minute, *count);
      if *count > highest_count {
        highest_count = *count;
        highest_minute = minute;
        highest_guard = guard;
      }
    }

    println!(
      "ID: {}, Minute: {}, Count: {}",
      highest_guard.id, highest_minute, highest_count
    );

    return (highest_minute * highest_guard.id).to_string();
  }
}

impl Day {
  fn parse_log(&self, line: &str) -> LogEntry {
    lazy_static! {
      static ref entry_re: Regex =
        Regex::new(r"^\[(\d{4}-\d{2}-\d{2}) (\d\d):(\d\d)\] (.+)$").unwrap();
      static ref shift_re: Regex = Regex::new(r"^Guard #(\d+) begins shift$").unwrap();
    }
    let mut date: String = String::from("");
    let mut hour: u32 = 0;
    let mut minute: u32 = 0;
    let mut text: String = String::from("");
    for cap in entry_re.captures_iter(line) {
      date = cap[1].to_string();
      hour = cap[2].parse().unwrap();
      minute = cap[3].parse().unwrap();
      text = cap[4].parse().unwrap();
      break;
    }
    let mut id = 0;
    let entry_type: LogEntryType = match text.as_ref() {
      "falls asleep" => LogEntryType::SLEEP,
      "wakes up" => LogEntryType::WAKE,
      _ => {
        for cap in shift_re.captures_iter(text.as_ref()) {
          id = cap[1].parse().unwrap();
          break;
        }
        LogEntryType::SHIFT
      }
    };
    return LogEntry {
      entry_type,
      date,
      hour,
      minute,
      id,
    };
  }

  fn parse_guards(&self, input: String) -> HashMap<u32, Guard> {
    let mut logs: Vec<LogEntry> = Vec::new();
    for line in input.lines() {
      let entry = self.parse_log(line);
      logs.push(entry);
    }
    logs.sort_by(|a, b| a.serialize().cmp(&b.serialize()));
    let mut guard_map: HashMap<u32, Guard> = HashMap::new();
    let mut id: u32 = 0;
    let mut asleep = false;
    let mut sleep_time: u32 = 0;
    for entry in logs {
      let mut guard = guard_map.entry(id).or_insert(Guard {
        minutes_asleep: 0,
        minute_map: HashMap::new(),
        id,
      });
      match entry.entry_type {
        LogEntryType::SHIFT => {
          if asleep {
            let minutes = 60 - sleep_time;
            guard.minutes_asleep += minutes;
            for i in sleep_time..60 {
              let count = guard.minute_map.entry(i).or_insert(0);
              *count += 1;
            }
          }
          id = entry.id;
          asleep = false;
          sleep_time = 0;
        }
        LogEntryType::SLEEP => {
          asleep = true;
          sleep_time = entry.minute;
        }
        LogEntryType::WAKE => {
          asleep = false;
          let minutes = 60 - sleep_time;
          guard.minutes_asleep += minutes;
          for i in sleep_time..entry.minute {
            let count = guard.minute_map.entry(i).or_insert(0);
            *count += 1;
          }

          // println!(
          //   "Guard #{} woke up. They slept for {}, and have slept a total of {}",
          //   id, minutes, guard.minutes_asleep
          // );
        }
      }
    }

    return guard_map;
  }
}
