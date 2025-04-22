pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

use regex::Regex;
pub fn part1(input: String) -> String {
    let re = Regex::new(r"-?[1-9][0-9]*").unwrap();
    re.find_iter(&input)
        .map(|x| x.as_str().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .into_iter()
        .sum::<i32>()
        .to_string()
}

use serde_json::Value;

fn walk_json(value: &Value, count: &mut i32) {
    match value {
        Value::Object(map) => {
            if !map
                .values()
                .any(|v| matches!(v, Value::String(s) if s == "red"))
            {
                for (_, val) in map {
                    walk_json(val, count);
                }
            }
        }
        Value::Array(arr) => {
            for val in arr.iter() {
                walk_json(val, count);
            }
        }
        Value::Number(n) => {
            *count += n.as_i64().unwrap() as i32;
        }
        _ => {}
    }
}

pub fn part2(input: String) -> String {
    let mut count: i32 = 0;
    let v: Value = serde_json::from_str(&input).unwrap();
    walk_json(&v, &mut count);
    count.to_string()
}
