use std::collections::HashMap;

pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

// ============================================================
// Parsing
// ============================================================

fn parse_input(input: &str) -> HashMap<String, HashMap<String, isize>> {
    let mut map: HashMap<String, HashMap<String, isize>> = HashMap::new();

    for line in input.lines() {
        let clean = line
            .replace("would ", "")
            .replace("gain ", "")
            .replace("lose ", "-")
            .replace(".", "")
            .replace("happiness units by sitting next to ", "");

        let mut parts = clean.splitn(3, ' ');

        let p1 = parts.next().unwrap();
        let val = parts.next().unwrap().parse::<isize>().unwrap();
        let p2 = parts.next().unwrap();

        map.entry(p1.to_string())
            .or_default()
            .insert(p2.to_string(), val);
    }

    map
}

// ============================================================
// Generic permutation generator
// ============================================================

fn permutations<T: Clone>(items: &mut Vec<T>, start: usize, acc: &mut Vec<Vec<T>>) {
    if start == items.len() {
        acc.push(items.clone());
        return;
    }

    for i in start..items.len() {
        items.swap(start, i);
        permutations(items, start + 1, acc);
        items.swap(start, i);
    }
}

// ============================================================
// Happiness calculation
// ============================================================

fn calculate_happiness(perm: &[String], map: &HashMap<String, HashMap<String, isize>>) -> isize {
    let n = perm.len();
    let mut sum = 0;

    for i in 0..n {
        let left = if i == 0 { n - 1 } else { i - 1 };
        let right = (i + 1) % n;

        let me = &perm[i];
        let left_p = &perm[left];
        let right_p = &perm[right];

        sum += map[me][left_p];
        sum += map[me][right_p];
    }

    sum
}

// ============================================================
// Part 1
// ============================================================

pub fn part1(input: String) -> String {
    let map = parse_input(&input);

    // Extract names
    let names: Vec<String> = map.keys().cloned().collect();

    // Generate permutations (fix the first person to avoid duplicate circular rotations)
    let fixed = names[0].clone();
    let mut rest: Vec<String> = names[1..].to_vec();

    let mut perms = Vec::new();
    permutations(&mut rest, 0, &mut perms);

    // Evaluate all rotations with fixed first seat
    let mut best = isize::MIN;

    for mut p in perms {
        p.insert(0, fixed.clone()); // place fixed person at head
        let score = calculate_happiness(&p, &map);
        best = best.max(score);
    }

    best.to_string()
}

// ============================================================
// Part 2 ("me" with zero happiness)
// ============================================================

pub fn part2(input: String) -> String {
    let mut map = parse_input(&input);

    let me = "Me".to_string();

    // Add "Me" with zero happiness toward everyone
    for person in map.clone().keys() {
        map.entry(person.clone())
            .or_insert_with(HashMap::new)
            .insert(me.clone(), 0);

        map.entry(me.clone())
            .or_insert_with(HashMap::new)
            .insert(person.clone(), 0);
    }

    // Extract names
    let names: Vec<String> = map.keys().cloned().collect();

    // Fix first person to avoid circular duplicates
    let fixed = names[0].clone();
    let mut rest: Vec<String> = names[1..].to_vec();

    let mut perms = Vec::new();
    permutations(&mut rest, 0, &mut perms);

    let mut best = isize::MIN;

    for mut p in perms {
        p.insert(0, fixed.clone());
        let score = calculate_happiness(&p, &map);
        best = best.max(score);
    }

    best.to_string()
}
