pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

pub fn part1(input: String) -> String {
    input.to_owned()
}
pub fn part2(input: String) -> String {
    input.to_owned()
}
