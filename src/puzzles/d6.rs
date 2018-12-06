use puzzles::puzzle::Puzzle;
use std::collections::HashMap;

pub struct Day {}

impl Puzzle for Day {
  fn desc(&self) -> (String, String) {
    return (
      String::from("Find furthest points"),
      String::from("Find closest points"),
    );
  }

  fn puzzle_a(&self, input: String) -> String {
    let mut max_point = Point {
      x: 0,
      y: 0,
      valid: true,
      area: 0,
    };
    let mut points = self.parse_points(&input, &mut max_point);

    self.check_points(&mut points, &max_point);

    let mut largest_area: i32 = 0;
    let mut largest_point = Point {
      x: 0,
      y: 0,
      area: 0,
      valid: false,
    };
    for (_, point) in points {
      // println!("{}: {}, {}", point.serialize(), point.area, point.valid);
      if point.valid && point.area > largest_area {
        largest_area = point.area;
        largest_point = point;
      }
    }
    return largest_point.area.to_string();
  }

  fn puzzle_b(&self, input: String) -> String {
    let mut max_point = Point {
      x: 0,
      y: 0,
      valid: true,
      area: 0,
    };
    let points = self.parse_points(&input, &mut max_point);

    let mut area_sum: i32 = 0;
    for x in 0..=max_point.x {
      for y in 0..=max_point.y {
        let mut target = Point {
          x,
          y,
          valid: false,
          area: 0,
        };
        let mut distance_sum: i32 = 0;
        for (_, point) in &points {
          distance_sum += target.distance(&point);
        }
        if distance_sum < 10000 {
          area_sum += 1;
        }
      }
    }
    return area_sum.to_string();
  }
}

impl Day {
  fn parse_points(&self, input: &String, max_point: &mut Point) -> HashMap<String, Point> {
    let mut points: HashMap<String, Point> = HashMap::new();

    for line in input.lines() {
      let parts: Vec<&str> = line.split(", ").collect();
      let mut point = Point {
        x: parts[0].parse().unwrap(),
        y: parts[1].parse().unwrap(),
        valid: true,
        area: 0,
      };
      if point.x > max_point.x {
        max_point.x = point.x;
      }
      if point.y > max_point.y {
        max_point.y = point.y;
      }
      points.insert(point.serialize(), point);
    }

    return points;
  }

  fn check_points(&self, points: &mut HashMap<String, Point>, max_point: &Point) {
    for x in 0..=max_point.x {
      for y in 0..=max_point.y {
        let mut point = Point {
          x,
          y,
          valid: false,
          area: 0,
        };
        let p = self.check_point(&point, &points, &max_point);
        if points.contains_key(&p.serialize()) {
          points.insert(p.serialize(), p);
        }
      }
    }
  }

  fn check_point(&self, target: &Point, points: &HashMap<String, Point>, max: &Point) -> Point {
    let mut matches: Vec<Point> = Vec::new();
    // shrug
    let mut smallest: i32 = 9999;
    for (_, point) in points {
      if target.x == point.x && target.y == point.y {
        continue;
      }
      let distance = target.distance(&point);

      if distance < smallest {
        matches.clear();
        smallest = distance;
        // println!(
        //   "*{}: {}, {} | {}",
        //   point.serialize(),
        //   point.area,
        //   point.valid,
        //   distance
        // );

        matches.push(point.clone());
      } else if distance == smallest {
        matches.push(point.clone());
      }
    }
    // println!("============");
    let mut point: Point;
    if matches.len() == 1 {
      point = matches[0].clone();
      point.area += 1;
    } else {
      point = Point {
        x: 0,
        y: 0,
        area: 0,
        valid: false,
      };
    }

    // println!(
    //   "{}: {}, {} | {}",
    //   matches[0].serialize(),
    //   matches[0].area,
    //   matches[0].valid,
    //   matches.len()
    // );

    if target.x == 0 || target.y == 0 || target.x == max.x || target.y == max.y {
      // println!("Marked invalid");
      point.valid = false;
    }
    return point;
  }
}

struct Point {
  x: i32,
  y: i32,
  valid: bool,
  area: i32,
}

impl Point {
  fn distance(&self, target: &Point) -> i32 {
    return (self.x - target.x).abs() + (self.y - target.y).abs();
  }

  fn serialize(&self) -> String {
    return format!("{},{}", self.x, self.y);
  }

  fn clone(&self) -> Point {
    Point {
      x: self.x,
      y: self.y,
      valid: self.valid,
      area: self.area,
    }
  }
}
