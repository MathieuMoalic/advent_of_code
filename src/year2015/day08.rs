pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

use regex::Regex;
pub fn part1(input: String) -> String {
    let mut str_len_total = 0;
    let mut char_len_total = 0;

    let re_hex = Regex::new(r"\\x([0-9A-Fa-f]{2})").unwrap();
    let re_slash = Regex::new(r"\\\\").unwrap();
    let re_apos = Regex::new(r#"\\\""#).unwrap();

    for line in input.lines() {
        let str_len = line.len();

        let line = &line[1..line.len() - 1];
        let line = re_hex.replace_all(line, |caps: &regex::Captures| {
            let byte = u8::from_str_radix(&caps[1], 16).unwrap();
            (byte as char).to_string()
        });
        let line = re_slash.replace_all(&line, r"\");
        let line = re_apos.replace_all(&line, r#"""#);

        let char_len = line.chars().count();
        str_len_total += str_len;
        char_len_total += char_len;
    }
    (str_len_total - char_len_total).to_string()
}

pub fn part2(input: String) -> String {
    let mut str_len_total = 0;
    let mut char_len_total = 0;

    let re_apos = Regex::new(r#"\""#).unwrap();
    let re_slash = Regex::new(r#"\\"#).unwrap();

    for line in input.lines() {
        let str_len = line.len();

        let line = re_slash.replace_all(line, r#"\\"#);
        let line = re_apos.replace_all(&line, r#"\""#);

        let char_len = line.chars().count() + 2;
        str_len_total += str_len;
        char_len_total += char_len;
    }
    (char_len_total - str_len_total).to_string()
}
