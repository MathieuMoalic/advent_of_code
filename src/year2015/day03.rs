pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

pub fn part1(input: String) -> String {
    let mut x: usize = 256;
    let mut y: usize = 256;
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 512]; 512];
    grid[256][256] = 1;
    for c in input.chars() {
        match c {
            '<' => x -= 1,
            '>' => x += 1,
            'v' => y -= 1,
            '^' => y += 1,
            _ => panic!("wtf"),
        }
        grid[y][x] += 1
    }

    let non_zero_count = grid.iter().flatten().filter(|&&val| val != 0).count();
    non_zero_count.to_string()
}

#[derive(PartialEq, Clone, Copy)]
struct Santa {
    x: usize,
    y: usize,
}

pub fn part2(input: String) -> String {
    let mut santa = Santa { x: 256, y: 256 };
    let mut robot = Santa { x: 256, y: 256 };
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 512]; 512];
    grid[256][256] = 2;
    for (i, c) in input.chars().enumerate() {
        let active = if i % 2 == 0 { &mut santa } else { &mut robot };

        match c {
            '<' => active.x -= 1,
            '>' => active.x += 1,
            'v' => active.y -= 1,
            '^' => active.y += 1,
            _ => panic!("Invalid input"),
        }

        grid[active.y][active.x] += 1;
    }

    let non_zero_count = grid.iter().flatten().filter(|&&val| val != 0).count();
    non_zero_count.to_string()
}
