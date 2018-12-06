#[macro_use]
extern crate lazy_static;
extern crate regex;

mod puzzles;

fn main() {
    println!("{yellow}@=========================@", yellow = "\x1B[33m");
    println!(
        "|   {red}Advent {green}Of {red}Code {cyan}2018   {yellow}|",
        yellow = "\x1B[33m",
        green = "\x1B[32m",
        red = "\x1B[31m",
        cyan = "\x1B[36m"
    );
    println!(
        "|          - {green}stupid {red}cat   {yellow}|",
        yellow = "\x1B[33m",
        green = "\x1B[32m",
        red = "\x1B[31m"
    );
    println!("@=========================@{reset}\n", reset = "\x1B[0m");

    let mut do_loop = true;
    while do_loop {
        do_loop = puzzles::execute_puzzle();
    }
}
