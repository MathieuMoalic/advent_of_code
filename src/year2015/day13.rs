use std::collections::HashMap;

pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

fn parse_input(input: String) -> HashMap<String, HashMap<String, isize>> {
    let mut v = HashMap::new();
    for line in input.lines() {
        let clean_line = line
            .replace("would ", "")
            .replace("gain ", "")
            .replace("lose ", "-")
            .replace(".", "")
            .replace("happiness units by sitting next to ", "");
        let mut words = clean_line.splitn(3, " ");

        let person1 = words.next().unwrap();
        let value = words.next().unwrap().parse::<isize>().unwrap();
        let person2 = words.next().unwrap().to_string();

        if !v.contains_key(person1) {
            v.insert(person1.to_string(), HashMap::new());
        }
        if let Some(val) = v.get_mut(person1) {
            val.insert(person2, value);
        };
    }
    v
}
fn permutations<T: Clone>(names: &mut Vec<T>, start: usize, acc: &mut Vec<Vec<T>>) {}

fn calculate_happiness(perm: Vec<&String>, map: &HashMap<String, HashMap<String, isize>>) -> isize {
    let mut h = 0;
    for (i, val) in perm.iter().enumerate() {
        let left = if i == 0 { 7 } else { i - 1 };
        let right = (i + 1) % 8;

        h += map[*val][*perm.get(left).unwrap()];
        h += map[*val][*perm.get(right).unwrap()];
    }
    h
}

pub fn part1(input: String) -> String {
    let v = parse_input(input);
    let mut keys: Vec<_> = v.keys().collect();

    let mut perms = Vec::new();
    perms.push(keys.clone());
    permutations(&mut keys, 0, &mut perms);

    let mut best_score: isize = 0;
    for p in perms {
        println!("{:?}", p);
        let h = calculate_happiness(p, &v);
        if h > best_score {
            best_score = h;
        }
    }
    //
    // let names = v.keys();
    // // for permutation in
    // for name in names {
    //     println!("{name}");
    // }
    best_score.to_string()
}
pub fn part2(input: String) -> String {
    input.to_owned()
}
