pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

pub fn part1(input: String) -> String {
    let mut floor: isize = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
    }
    floor.to_string()
}

pub fn part2(input: String) -> String {
    let mut floor: isize = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor < 0 {
            return (i + 1).to_string();
        }
    }
    String::from("Something went wrong")
}
