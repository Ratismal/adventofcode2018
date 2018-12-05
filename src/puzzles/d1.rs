use std::collections::HashMap;

use puzzles::puzzle::Puzzle;

pub struct Day {}

impl Puzzle for Day {
    fn desc(&self) -> (String, String) {
        return (
            String::from("Add a series of numbers"),
            String::from("Find first repeated sum"),
        );
    }

    fn puzzle_a(&self, input: String) -> String {
        let lines = input.lines();

        let mut result: i32 = 0;

        for line in lines {
            let i: i32 = line.parse().unwrap();
            result += i;
        }

        let res = result.to_string();

        return res;
    }

    fn puzzle_b(&self, input: String) -> String {
        let mut map = HashMap::new();

        let mut result: i32 = 0;
        let mut cont = true;

        while cont {
            let lines = input.lines();
            for line in lines {
                let mut i: i32 = line.parse().unwrap();
                result += i;

                if map.contains_key(&result) {
                    cont = false;
                    break;
                } else {
                    map.insert(result, true);
                }
            }
        }

        let res = result.to_string();

        return res;
    }
}
