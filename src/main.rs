#![allow(dead_code)]
use std::env;

mod year2015;
mod year2016;
mod year2017;
mod year2018;
mod year2019;
mod year2020;
mod year2021;
mod year2022;
mod year2023;
mod year2024;
mod year2025;

use std::fs;
use std::path::Path;
use std::time::Duration;

use reqwest::blocking::Client;
use reqwest::header::{COOKIE, USER_AGENT};

pub fn get_input(year: &usize, day: &usize) -> String {
    let file_path = format!("inputs/{year:04}_{day:02}.txt");

    if Path::new(&file_path).exists() {
        return fs::read_to_string(file_path).expect("Failed to read cached input");
    }

    let session = fs::read_to_string(".env")
        .expect("Missing session.txt with your AOC session cookie")
        .trim()
        .to_string();

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let response = client
        .get(&url)
        .header(
            USER_AGENT,
            "github.com/mathieumoalic/advent_of_code by mathieu@matmoa.eu",
        )
        .header(COOKIE, format!("session={}", session))
        .send()
        .expect("Failed to send request");

    if !response.status().is_success() {
        panic!("Failed to get input: status {}", response.status());
    }

    let input = response
        .text()
        .expect("Failed to read response text")
        .trim_end()
        .to_string();

    fs::create_dir_all("inputs").unwrap();
    fs::write(&file_path, &input).expect("Failed to write input to file");

    input
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprint!("Usage: cargo run -- <year> <day> <part>");
        return;
    };
    let year: usize = args[1]
        .parse()
        .expect("Year must be a number between 0–255");
    let day: usize = args[2].parse().expect("Day must be a number between 0–255");
    let part: usize = args[3]
        .parse()
        .expect("Part must be a number between 0–255");

    let input = get_input(&year, &day);
    let output = match year {
        2015 => year2015::run(day, part, input),
        2016 => year2016::run(day, part, input),
        2017 => year2017::run(day, part, input),
        2018 => year2018::run(day, part, input),
        2019 => year2019::run(day, part, input),
        2020 => year2020::run(day, part, input),
        2021 => year2021::run(day, part, input),
        2022 => year2022::run(day, part, input),
        2023 => year2023::run(day, part, input),
        2024 => year2024::run(day, part, input),
        2025 => year2025::run(day, part, input),
        _ => String::from("Wrong year"),
    };
    println!("{output}")
}
