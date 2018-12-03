use std::collections::HashMap;

pub fn puzzle_a(input: String) -> String {
    let lines = input.lines();

    let mut result: i32 = 0;

    println!("Adding a series of numbers...");

    for line in lines {
        let i: i32 = line.parse().unwrap();
        result += i;
    }

    let res = result.to_string();

    return res;
}

pub fn puzzle_b(input: String) -> String {
    let mut map = HashMap::new();

    let mut result: i32 = 0;
    let mut cont = true;

    println!("Finding first repeated result when continuously adding a series of numbers...");

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