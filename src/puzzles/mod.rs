use std::fs;
use std::io;
use std::io::stdout;
use std::io::Write;
use std::time::Instant;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod puzzle;

use self::puzzle::Puzzle;

pub fn execute_puzzle() -> bool {
    let mut days: Vec<Box<Puzzle>> = Vec::new();
    days.push(Box::new(d1::Day {}));
    days.push(Box::new(d2::Day {}));
    days.push(Box::new(d3::Day {}));
    days.push(Box::new(d4::Day {}));
    days.push(Box::new(d5::Day {}));
    days.push(Box::new(d6::Day {}));
    days.push(Box::new(d7::Day {}));

    // todo: better menu
    println!("Please select from the list:");
    for index in 0..days.len() {
        let day = &days[index as usize];
        let (desc_a, desc_b) = day.desc();
        let _i = index + 1;
        println!(
            "  Day {id:02}: | {id}a. {a:30}| {id}b. {b}",
            id = _i,
            a = desc_a,
            b = desc_b
        );
    }

    println!("  q. Quit the application");

    print!("> ");

    let _ = stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    let trimmed = input.trim_right().to_string();

    if trimmed == String::from("q") {
        return false;
    }
    println!("Your choice: '{}'", trimmed);

    let id: String = trimmed.chars().take(trimmed.chars().count() - 1).collect();
    let puzzle_id: String = trimmed
        .chars()
        .skip(trimmed.chars().count() - 1)
        .take(1)
        .collect();

    println!("Day: {}, Puzzle: {}", id, puzzle_id);

    let i: usize = id.parse().expect("invalid number provided");

    if i >= days.len() {
        println!("Puzzle did not exist");
    } else {
        let filename = format!("input/d{}.txt", i);

        let content = fs::read_to_string(&filename).expect("Could not read puzzle input");

        let day_struct = &days[(i - 1) as usize];

        let start = Instant::now();
        let res: String = match puzzle_id.as_ref() {
            "a" => day_struct.puzzle_a(content),
            "b" => day_struct.puzzle_b(content),
            _ => String::from("Invalid Puzzle"),
        };
        let end = Instant::now();

        println!("Result: {}", res);
        println!("Execution Time: {:?}", end.duration_since(start));
    }

    println!("Press enter to continue...");
    let mut _input = String::new();
    io::stdin()
        .read_line(&mut _input)
        .expect("error: unable to read user input");

    return true;
}
