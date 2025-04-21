pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

pub fn part1(input: String) -> String {
    let mut digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    for _ in 0..40 {
        let mut new_digits: Vec<u32> = Vec::new();
        let mut last_digit = digits[0];
        let mut digit_count = 1;
        (1..digits.len()).for_each(|i| {
            let d = digits[i];
            if d == last_digit {
                digit_count += 1
            } else {
                new_digits.push(digit_count);
                new_digits.push(last_digit);
                last_digit = d;
                digit_count = 1;
            }
        });
        new_digits.push(digit_count);
        new_digits.push(last_digit);
        digits = new_digits;
    }
    digits.len().to_string()
}

pub fn part2(input: String) -> String {
    let mut digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    for _ in 0..50 {
        let mut new_digits: Vec<u32> = Vec::new();
        let mut last_digit = digits[0];
        let mut digit_count = 1;
        (1..digits.len()).for_each(|i| {
            let d = digits[i];
            if d == last_digit {
                digit_count += 1
            } else {
                new_digits.push(digit_count);
                new_digits.push(last_digit);
                last_digit = d;
                digit_count = 1;
            }
        });
        new_digits.push(digit_count);
        new_digits.push(last_digit);
        digits = new_digits;
    }
    digits.len().to_string()
}
