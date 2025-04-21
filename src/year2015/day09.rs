pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

use itertools::Itertools;
use std::collections::HashMap;

fn make_city_ids() -> HashMap<String, usize> {
    let mut city_ids = HashMap::new();

    city_ids.insert("Faerun".to_string(), 0);
    city_ids.insert("Tristram".to_string(), 1);
    city_ids.insert("Tambi".to_string(), 2);
    city_ids.insert("Norrath".to_string(), 3);
    city_ids.insert("Snowdin".to_string(), 4);
    city_ids.insert("Straylight".to_string(), 5);
    city_ids.insert("AlphaCentauri".to_string(), 6);
    city_ids.insert("Arbre".to_string(), 7);

    city_ids
}

fn parse_input(input: String) -> [[usize; 8]; 8] {
    let mut cities: [[usize; 8]; 8] = [[0; 8]; 8];
    let city_ids = make_city_ids();
    for line in input.lines() {
        let strings: Vec<&str> = line.splitn(5, " ").collect();
        let c1 = city_ids.get(strings[0]).unwrap().to_owned();
        let c2 = city_ids.get(strings[2]).unwrap().to_owned();
        let distance = strings[4].parse::<usize>().unwrap();
        cities[c1][c2] = distance;
        cities[c2][c1] = distance;
    }
    cities
}

pub fn part1(input: String) -> String {
    let cities = parse_input(input);
    let mut min_distance = usize::MAX;
    let v: Vec<usize> = (0..=7).collect();
    for perm in v.iter().permutations(v.len()) {
        let mut distance = 0;
        for i in 0..7 {
            distance += cities[*perm[i]][*perm[i + 1]]
        }
        if distance < min_distance {
            min_distance = distance
        }
    }
    min_distance.to_string()
}

pub fn part2(input: String) -> String {
    let cities = parse_input(input);
    let mut max_distance = 0;
    let v: Vec<usize> = (0..=7).collect();
    for perm in v.iter().permutations(v.len()) {
        let mut distance = 0;
        for i in 0..7 {
            distance += cities[*perm[i]][*perm[i + 1]]
        }
        if distance > max_distance {
            max_distance = distance
        }
    }
    max_distance.to_string()
}
