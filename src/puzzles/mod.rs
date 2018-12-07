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

const RESET: &str = "\x1B[0m";

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
            "{escape}  Day {id:02}: | {id}a. {a:30}| {id}b. {b}{reset}",
            escape = if index % 2 == 0 {
                "\x1B[31m"
            } else {
                "\x1B[32m"
            },
            id = _i,
            a = desc_a,
            b = desc_b,
            reset = RESET
        );
    }

    println!("\x1B[35m  q. Quit the application{}", RESET);

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
    println!("\x1B[33m| Your choice: \x1B[36m'{}'", trimmed);

    let id: String = trimmed.chars().take(trimmed.chars().count() - 1).collect();
    let puzzle_id: String = trimmed
        .chars()
        .skip(trimmed.chars().count() - 1)
        .take(1)
        .collect();

    println!(
        "\x1B[33m| Day: \x1B[36m{}\x1B[33m, Puzzle: \x1B[36m{}",
        id, puzzle_id
    );

    let i: usize = id.parse().expect("invalid number provided");

    if i > days.len() {
        println!("Puzzle did not exist");
    } else {
        let filename = format!("input/d{}.txt", i);

        let content = fs::read_to_string(&filename).expect("Could not read puzzle input");

        let day_struct = &days[(i - 1) as usize];

        let start = Instant::now();
        print!("\x1B[0m");
        let res: String = match puzzle_id.as_ref() {
            "a" => day_struct.puzzle_a(content),
            "b" => day_struct.puzzle_b(content),
            _ => String::from("Invalid Puzzle"),
        };
        let end = Instant::now();

        println!("\x1B[33m| Result: \x1B[36m{}", res);
        let duration = end.duration_since(start);
        println!("\x1B[33m| Execution Time: \x1B[36m{:?}", duration);
    }

    println!("\x1B[35mPress \x1B[43;30m enter \x1B[0;35m to continue...\x1B[0m");
    let mut _input = String::new();
    io::stdin()
        .read_line(&mut _input)
        .expect("error: unable to read user input");

    return true;
}
