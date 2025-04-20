pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

#[derive(Debug)]
enum Action {
    Toggle,
    On,
    Off,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

fn parse_instructions(input: String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let c7: char = line.chars().nth(6).unwrap();
        let action = match c7 {
            'n' => Action::On,
            'f' => Action::Off,
            ' ' => Action::Toggle,
            _ => panic!("What is this char {}", c7),
        };
        let sub_strings: Vec<&str> = line.split_whitespace().collect();
        let origin = match action {
            Action::Toggle => sub_strings[1],
            _ => sub_strings[2],
        };
        let destination = sub_strings[sub_strings.len() - 1];
        let mut a = origin.splitn(2, ',');
        let mut b = destination.splitn(2, ',');
        let value = Instruction {
            action,
            x1: a.next().unwrap().parse::<usize>().unwrap(),
            y1: a.next().unwrap().parse::<usize>().unwrap(),
            x2: b.next().unwrap().parse::<usize>().unwrap(),
            y2: b.next().unwrap().parse::<usize>().unwrap(),
        };
        instructions.push(value)
    }
    instructions
}

pub fn part1(input: String) -> String {
    let instructions = parse_instructions(input);
    let mut lights: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];
    for i in instructions {
        (i.x1..=i.x2).for_each(|x| {
            (i.y1..=i.y2).for_each(|y| match i.action {
                Action::On => lights[y][x] = true,
                Action::Off => lights[y][x] = false,
                Action::Toggle => lights[y][x] = !lights[y][x],
            });
        });
    }
    lights
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&b| b)
        .count()
        .to_string()
}
pub fn part2(input: String) -> String {
    let instructions = parse_instructions(input);
    let mut lights: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    for i in instructions {
        (i.x1..=i.x2).for_each(|x| {
            (i.y1..=i.y2).for_each(|y| match i.action {
                Action::On => lights[y][x] += 1,
                // this makes sure is is not below 0.
                Action::Off => lights[y][x] = lights[y][x].saturating_sub(1),
                Action::Toggle => lights[y][x] += 2,
            });
        });
    }
    lights
        .iter()
        .flat_map(|row| row.iter())
        .sum::<usize>()
        .to_string()
}
