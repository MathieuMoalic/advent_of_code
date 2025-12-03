pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

const FIRST_CODE: u64 = 20151125;
const MULTIPLIER: u64 = 252533;
const DIVISOR: u64 = 33554393;

fn get_row_col(input: &str) -> (u64, u64) {
    let nums: Vec<u64> = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    (nums[0], nums[1])
}

pub fn part1(input: String) -> String {
    let (row, col) = get_row_col(&input);
    let n = row + col - 1;
    let i = (n - 1) * n / 2 + col;
    let mut prev = FIRST_CODE;
    for _ in 1..i {
        prev = prev * MULTIPLIER % DIVISOR;
    }
    prev.to_string()
}
pub fn part2(input: String) -> String {
    input.to_owned()
}
