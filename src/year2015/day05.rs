pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

pub fn part1(input: String) -> String {
    let patterns = ["ab", "cd", "pq", "xy"];
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut count: usize = 0;
    for line in input.lines() {
        if patterns.iter().any(|pat| line.contains(pat)) {
            continue;
        }
        if line.chars().filter(|c| vowels.contains(c)).count() < 3 {
            continue;
        }
        let chars: Vec<char> = line.chars().collect();
        for i in 1..chars.len() {
            if chars[i - 1] == chars[i] {
                count += 1;
                break;
            }
        }
    }
    count.to_string()
}

pub fn part2(input: String) -> String {
    let mut count = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();

        let has_sandwich = (1..chars.len() - 1).any(|i| chars[i - 1] == chars[i + 1]);
        if !has_sandwich {
            continue;
        }

        let mut seen_pairs = std::collections::HashMap::new();
        let mut has_repeated_pair = false;

        for i in 0..chars.len() - 1 {
            let pair = (chars[i], chars[i + 1]);
            if let Some(&prev_index) = seen_pairs.get(&pair) {
                if i > prev_index + 1 {
                    has_repeated_pair = true;
                    break;
                }
            } else {
                seen_pairs.insert(pair, i);
            }
        }

        if has_repeated_pair {
            count += 1;
        }
    }

    count.to_string()
}
