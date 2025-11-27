pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}
fn parse_input(input: String) -> Lights {
    let mut lights = [[0; SIZE]; SIZE];

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            lights[i][j] = (char == '#') as usize;
        }
    }
    lights
}

fn count_neighbours(lights: &Lights, x: usize, y: usize) -> usize {
    let mut count = 0;

    for dx in -1isize..=1 {
        for dy in -1isize..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            // skip out-of-bounds indices
            if nx < 0 || ny < 0 {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);

            count += lights
                .get(nx)
                .and_then(|row| row.get(ny))
                .copied()
                .unwrap_or(0);
        }
    }

    count
}

fn pprint(lights: &Lights) {
    for row in lights {
        let s: String = row
            .iter()
            .map(|&x| if x == 1 { '#' } else { '.' })
            .collect();
        println!("{s}");
    }
    println!("--------------");
}
fn step(lights: &Lights, part2: bool) -> Lights {
    // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
    // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.

    let mut new_lights = [[0; SIZE]; SIZE];
    for x in 0..SIZE {
        for y in 0..SIZE {
            if part2 && (x == 0 || x == SIZE - 1) && (y == 0 || y == SIZE - 1) {
                new_lights[x][y] = 1;
                continue;
            }
            let count = count_neighbours(lights, x, y);
            if lights[x][y] == 1 && (count == 2 || count == 3) {
                new_lights[x][y] = 1;
            }
            if lights[x][y] == 0 && (count == 3) {
                new_lights[x][y] = 1;
            }
        }
    }
    new_lights
}
const SIZE: usize = 100;
const STEPS: usize = 100;
type Lights = [[usize; SIZE]; SIZE];

pub fn part1(input: String) -> String {
    let mut lights = parse_input(input);
    for _ in 0..STEPS {
        lights = step(&lights, false);
    }
    lights
        .map(|row| row.iter().sum())
        .iter()
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let mut lights = parse_input(input);
    lights[0][0] = 1;
    lights[0][SIZE - 1] = 1;
    lights[SIZE - 1][0] = 1;
    lights[SIZE - 1][SIZE - 1] = 1;
    for _ in 0..STEPS {
        lights = step(&lights, true);
    }
    lights
        .map(|row| row.iter().sum())
        .iter()
        .sum::<usize>()
        .to_string()
}
