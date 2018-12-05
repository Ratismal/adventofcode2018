mod puzzles;

fn main() {
    println!("@=========================@");
    println!("|   Advent Of Code 2018   |");
    println!("|          - stupid cat   |");
    println!("@=========================@\n");

    let mut do_loop = true;
    while do_loop {
        do_loop = puzzles::execute_puzzle();
    }
}
