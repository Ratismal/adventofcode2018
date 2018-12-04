mod d1;
mod d2;
mod puzzle;

fn missing_puzzle(_content: String) -> String {
    let ret = String::from("Puzzle did not exist.");
    return ret;
}

pub fn execute_puzzle(i: u32, content: String) -> String {
    // todo: dynamic matching
    let p_func = match i {
        1 => d1::puzzle_a,
        2 => d1::puzzle_b,
        3 => d2::puzzle_a,
        4 => d2::puzzle_b,
        _ => missing_puzzle,
    };

    let res = p_func(content);

    return res;
}
