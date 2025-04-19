pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

pub fn part1(input: String) -> String {
    let mut total: usize = 0;
    for line in input.lines() {
        let dims: Vec<usize> = line
            .split('x')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let (h, l, w) = (dims[0], dims[1], dims[2]);
        let sides = [h * l, w * l, h * w];
        total += sides.iter().sum::<usize>() * 2;
        total += *sides.iter().min().unwrap();
    }
    total.to_string()
}

pub fn part2(input: String) -> String {
    let mut total: usize = 0;
    for line in input.lines() {
        let mut dims: Vec<usize> = line
            .split('x')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        dims.sort();
        total += dims[0] * 2 + dims[1] * 2;
        total += dims.iter().product::<usize>();
    }
    total.to_string()
}
