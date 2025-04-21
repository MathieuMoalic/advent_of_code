pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

fn rule1_ok(v: &[usize]) -> bool {
    for w in v.windows(3) {
        if w[1] == w[0] + 1 && w[2] == w[1] + 1 {
            return true;
        }
    }
    false
}
fn rule2_ok(v: &Vec<usize>) -> bool {
    let forbidden: Vec<usize> = "iol".chars().map(|c| (c as u8 - b'a') as usize).collect();
    for i in v {
        if forbidden.contains(i) {
            return false;
        }
    }
    true
}
fn rule3_ok(v: &Vec<usize>) -> bool {
    let mut pair_count = 0;
    let mut last_letter = 50; // 50 is not a possible letter so it will never match
    for i in v {
        if *i == last_letter {
            pair_count += 1;
            last_letter = 50;
        } else {
            last_letter = *i;
        }
        if pair_count > 1 {
            return true;
        }
    }
    false
}

fn increment_pw(old_pw: Vec<usize>) -> Vec<usize> {
    let mut new_pw = vec![0; old_pw.len()];
    new_pw[old_pw.len() - 1] = 1;
    for (i, val) in old_pw.iter().enumerate().rev() {
        let new_letter = val + new_pw[i];
        if new_letter == 26 {
            new_pw[i - 1] = 1;
            new_pw[i] = 0;
        } else {
            new_pw[i] = new_letter;
        }
    }
    new_pw
}

pub fn part1(input: String) -> String {
    let mut v: Vec<usize> = input.chars().map(|c| (c as u8 - b'a') as usize).collect();
    while !(rule1_ok(&v) && rule2_ok(&v) && rule3_ok(&v)) {
        v = increment_pw(v);
    }
    v.iter().map(|&n| (b'a' + n as u8) as char).collect()
}

pub fn part2(input: String) -> String {
    let mut v: Vec<usize> = input.chars().map(|c| (c as u8 - b'a') as usize).collect();
    while !(rule1_ok(&v) && rule2_ok(&v) && rule3_ok(&v)) {
        v = increment_pw(v);
    }
    v = increment_pw(v);
    while !(rule1_ok(&v) && rule2_ok(&v) && rule3_ok(&v)) {
        v = increment_pw(v);
    }
    v.iter().map(|&n| (b'a' + n as u8) as char).collect()
}
