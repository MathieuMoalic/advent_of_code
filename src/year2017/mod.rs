pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub fn run(day: usize, part: usize, input: String) -> String {
    match day {
        1 => day01::run(part, input),
        2 => day02::run(part, input),
        3 => day03::run(part, input),
        4 => day04::run(part, input),
        5 => day05::run(part, input),
        6 => day06::run(part, input),
        7 => day07::run(part, input),
        8 => day08::run(part, input),
        9 => day09::run(part, input),
        10 => day10::run(part, input),
        11 => day11::run(part, input),
        12 => day12::run(part, input),
        13 => day13::run(part, input),
        14 => day14::run(part, input),
        15 => day15::run(part, input),
        16 => day16::run(part, input),
        17 => day17::run(part, input),
        18 => day18::run(part, input),
        19 => day19::run(part, input),
        20 => day20::run(part, input),
        21 => day21::run(part, input),
        22 => day22::run(part, input),
        23 => day23::run(part, input),
        24 => day24::run(part, input),
        25 => day25::run(part, input),
        _ => panic!("Invalid day: {}", day),
    }
}
