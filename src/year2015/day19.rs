use std::collections::HashMap;

pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

fn parse_input(input: String) -> (HashMap<String, Vec<String>>, Vec<String>) {
    let mut r = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let words = line.split(" => ").collect::<Vec<&str>>();
        let a = words[0].to_string();
        let b = words[1].to_string();

        r.entry(a).or_insert_with(Vec::new).push(b);
    }
    let mut v = vec![];
    let chars: Vec<char> = input.lines().last().unwrap().chars().collect();
    for i in 0..chars.len() {
        let ch = chars.get(i).unwrap();
        if ch.is_lowercase() {
            continue;
        } else {
            if i == chars.len() {
                continue;
            }
            let mut s = ch.to_string();
            if i < chars.len() - 1 {
                let ch_next = chars.get(i + 1).unwrap();
                if ch_next.is_lowercase() {
                    s.push(*ch_next);
                }
            }
            v.push(s);
        }
    }
    (r, v)
}

pub fn part1(input: String) -> String {
    let (replacements, molecule) = parse_input(input);
    let mut molecules: Vec<String> = Vec::new();
    for i in 0..molecule.len() {
        let element = molecule[i].clone();
        let v = Vec::new();
        for el_rep in replacements.get(&element).unwrap_or(&v) {
            let mut new_molecule = molecule.clone();
            new_molecule[i] = el_rep.clone();
            let s = new_molecule.concat();
            if !molecules.contains(&s) {
                molecules.push(s);
            }
        }
    }
    molecules.len().to_string()
}

fn parse_input2(input: String) -> (Vec<(String, String)>, String) {
    let mut r = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let words = line.split(" => ").collect::<Vec<&str>>();
        let a = words[0].to_string();
        let b = words[1].to_string();
        r.push((b, a));
    }
    let v = input.lines().last().unwrap().to_string();
    (r, v)
}

pub fn part2(input: String) -> String {
    let (replacements, mut molecule) = parse_input2(input);
    let mut count = 0;
    while molecule != *"e" {
        for (from, to) in &replacements {
            let new_molecule = molecule.replacen(from, to, 1);
            if new_molecule != molecule {
                count += 1;
            }
            molecule = new_molecule;
        }
    }

    count.to_string()
}
