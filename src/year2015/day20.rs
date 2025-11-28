pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}
const MAX_HOUSE: usize = 1_000_000;
const MAX_VISITS: usize = 50;

pub fn part1(input: String) -> String {
    let goal = input.parse::<usize>().unwrap() / 10;
    let mut houses = vec![0usize; MAX_HOUSE + 1];

    for elf in 1..=MAX_HOUSE {
        for house in (elf..=MAX_HOUSE).step_by(elf) {
            houses[house] += elf;
        }
    }

    houses
        .iter()
        .enumerate()
        .skip(1) // ignore fake house 0
        .find(|&(_house, &presents)| presents >= goal)
        .map(|(house, _)| house)
        .unwrap()
        .to_string()
}

pub fn part2(input: String) -> String {
    let goal = input.parse::<usize>().unwrap();
    let mut houses = vec![0usize; MAX_HOUSE + 1];

    for elf in 1..=MAX_HOUSE {
        for (visits, house) in (elf..=MAX_HOUSE).step_by(elf).enumerate() {
            if visits == MAX_VISITS {
                break;
            }
            houses[house] += elf * 11;
        }
    }

    houses
        .iter()
        .enumerate()
        .skip(1) // ignore fake house 0
        .find(|&(_house, &presents)| presents >= goal)
        .map(|(house, _)| house)
        .unwrap()
        .to_string()
}
