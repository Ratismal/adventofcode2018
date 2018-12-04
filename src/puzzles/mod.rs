mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod puzzle;

pub fn execute_puzzle(i: u32, content: String) -> String {
    let funcs = [
        d1::puzzle_a,
        d1::puzzle_b,
        d2::puzzle_a,
        d2::puzzle_b,
        d3::puzzle_a,
        d3::puzzle_b,
        d4::puzzle_a,
        d4::puzzle_b,
        d5::puzzle_a,
        d5::puzzle_b,
        d6::puzzle_a,
        d6::puzzle_b,
        d7::puzzle_a,
        d7::puzzle_b,
    ];
    let p_func = funcs.get((i - 1) as usize).expect("Puzzle did not exist.");
    let res = p_func(content);

    return res;
}
