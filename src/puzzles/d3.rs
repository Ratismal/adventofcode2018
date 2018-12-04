use std::collections::HashMap;

struct PointPair {
  x: i32,
  y: i32,
}

impl PointPair {
  fn serialize(&self) -> String {
    return format!("{}-{}", &self.x, &self.y);
  }
}

struct FabricPoint {
  count: i32,
  id: Vec<i32>,
}

fn split(line: &str) -> (i32, PointPair, PointPair) {
  let snipped: String = line.chars().skip(1).take(line.len() - 1).collect();
  let sn1: Vec<&str> = snipped.split(" @ ").collect();
  let id: i32 = sn1[0].parse().unwrap();
  let sn2: Vec<&str> = sn1[1].split(": ").collect();
  let sn3: Vec<&str> = sn2[0].split(",").collect();
  let sn4: Vec<&str> = sn2[1].split("x").collect();
  let pos = PointPair {
    x: sn3[0].parse().unwrap(),
    y: sn3[1].parse().unwrap(),
  };
  let dim = PointPair {
    x: sn4[0].parse().unwrap(),
    y: sn4[1].parse().unwrap(),
  };
  return (id, pos, dim);
}

fn populate(input: String) -> HashMap<String, FabricPoint> {
  let mut map: HashMap<String, FabricPoint> = HashMap::new();
  for line in input.lines() {
    let (id, pos, dim) = split(line);
    for x in pos.x..(pos.x + dim.x) {
      for y in pos.y..(pos.y + dim.y) {
        let point = PointPair { x, y };
        let count = map.entry(point.serialize()).or_insert(FabricPoint {
          count: 0,
          id: [].to_vec(),
        });
        count.count += 1;
        count.id.push(id);
      }
    }
  }
  return map;
}

pub fn puzzle_a(input: String) -> String {
  let map = populate(input);
  let mut count: i32 = 0;
  for (_, val) in &map {
    if val.count > 1i32 {
      count += 1;
    }
  }
  return count.to_string();
}

pub fn puzzle_b(input: String) -> String {
  let mut map = populate(input.to_string());
  let mut id: i32 = 0;
  for line in input.lines() {
    let (did, pos, dim) = split(line);
    let mut good = true;
    for x in pos.x..(pos.x + dim.x) {
      for y in pos.y..(pos.y + dim.y) {
        let point = PointPair { x, y };
        let val = map.entry(point.serialize()).or_insert(FabricPoint {
          count: 0,
          id: [].to_vec(),
        });
        if val.count != 1 {
          good = false;
          break;
        }
      }
      if good == false {
        break;
      }
    }
    if good == true {
      id = did;
      break;
    }
  }
  return id.to_string();
}
