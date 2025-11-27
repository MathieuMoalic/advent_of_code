pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

const TOTAL_VOLUME: usize = 150;

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn rec(
    mut containers: Vec<usize>,
    combination: &mut Vec<usize>,
    combinations: &mut Vec<Vec<usize>>,
) {
    let sum: usize = combination.iter().sum();

    if sum == TOTAL_VOLUME {
        combinations.push(combination.clone());
        return;
    }
    if sum > TOTAL_VOLUME {
        return;
    }
    while let Some(container) = containers.pop() {
        combination.push(container);
        rec(containers.clone(), combination, combinations);
        combination.pop();
    }
}

pub fn part1(input: String) -> String {
    let containers = parse_input(&input);
    let mut combinations = Vec::new();
    let mut combination = Vec::new();
    rec(containers, &mut combination, &mut combinations);
    combinations.len().to_string()
}
pub fn part2(input: String) -> String {
    let containers = parse_input(&input);
    let mut combinations = Vec::new();
    let mut combination = Vec::new();
    rec(containers, &mut combination, &mut combinations);
    let min_size = combinations.iter().map(|x| x.len()).min().unwrap();
    combinations
        .into_iter()
        .filter(|x| x.len() == min_size)
        .collect::<Vec<Vec<usize>>>()
        .len()
        .to_string()
}
