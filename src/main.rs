use std::fs;
use std::io;
use std::io::stdout;
use std::io::Write;

mod puzzles;

fn main() {
    println!("@=========================@");
    println!("|   Advent Of Code 2018   |");
    println!("|          - stupid cat   |");
    println!("@=========================@\n");

    // todo: better menu
    println!("Please select from the list:");
    println!(" - Day 1: 1. Puzzle A, 2. Puzzle B");
    println!(" - Day 2: 3. Puzzle A, 4. Puzzle B");
    println!(" - Day 3: 5. Puzzle A, 6. Puzzle B");
    println!(" - Day 4: 7. Puzzle A, 8. Puzzle B");
    println!(" - Day 5: 9. Puzzle A, 10. Puzzle B");
    println!(" - Day 6: 11. Puzzle A, 12. Puzzle B");
    println!(" - Day 7: 13. Puzzle A, 14. Puzzle B");

    print!("> ");

    let _ = stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    let s = input.trim_right();

    println!("Your choice: '{}'", s);

    let i: u32 = s.parse().expect("invalid number provided");

    let day = ((i - 1) / 2) + 1;
    let puz = ((i - 1) % 2) + 1;

    println!("Index: {}, Day: {}, Puzzle: {}", i, day, puz);

    let filename = format!("input/d{}.txt", day);

    let contents = fs::read_to_string(&filename).expect("Could not read puzzle input");

    let res = puzzles::execute_puzzle(i, contents);

    println!("Result: {}", res);
}
