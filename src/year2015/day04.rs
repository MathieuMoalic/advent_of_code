pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

pub fn part1(input: String) -> String {
    let mut str_to_hash = String::with_capacity(input.len() + 20);
    for i in 0..usize::MAX {
        str_to_hash.clear();
        str_to_hash.push_str(&input);
        str_to_hash.push_str(&i.to_string());
        let hash = md5::compute(&str_to_hash).0;

        // hash[0] => bits 0..8
        // hash[1] => bits 8..16
        // hash[2] => bits 16..24
        if hash[0] == 0 && hash[1] == 0 && (hash[2] >> 4) == 0 {
            return i.to_string();
        }
    }
    panic!("Couldnt find answer")
}

use std::fmt::Write;
pub fn part2(input: String) -> String {
    let mut str_to_hash = String::with_capacity(input.len() + 20);
    let mut num_buf = String::with_capacity(20);

    for i in 0..usize::MAX {
        str_to_hash.clear();
        str_to_hash.push_str(&input);

        num_buf.clear();
        write!(&mut num_buf, "{}", i).unwrap();
        str_to_hash.push_str(&num_buf);

        let hash = md5::compute(&str_to_hash).0;

        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return i.to_string();
        }
    }
    panic!("Couldnt find answer")
}
