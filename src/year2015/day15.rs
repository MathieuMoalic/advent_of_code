pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}
#[derive(Debug)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

fn parse_input(input: &str) -> Vec<Ingredient> {
    let mut v = Vec::new();
    for line in input.lines() {
        let line = line.replace(",", "");
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let capacity = words[2].parse().unwrap();
        let durability = words[4].parse().unwrap();
        let flavor = words[6].parse().unwrap();
        let texture = words[8].parse().unwrap();
        let calories = words[10].parse().unwrap();
        v.push(Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
    v
}
const TOTAL_TSPS: usize = 100;
const TOTAL_CALORIES: isize = 500;

fn calculate_score(ings: &[Ingredient], tsps: &[isize; 4], count_calories: bool) -> isize {
    let mut score: [isize; 4] = [0, 0, 0, 0];
    let mut calories = 0;
    for (ing, tsp) in ings.iter().zip(tsps) {
        score[0] += ing.capacity * tsp;
        score[1] += ing.durability * tsp;
        score[2] += ing.flavor * tsp;
        score[3] += ing.texture * tsp;
        calories += ing.calories * tsp
    }
    if count_calories && calories != TOTAL_CALORIES {
        return 0;
    }
    for i in 0..score.len() {
        score[i] = 0.max(score[i]);
    }
    score.iter().product()
}

fn iterate_tsp_count(ings: &[Ingredient], count_calories: bool) -> isize {
    let mut max_score = isize::MIN;
    for i in 0..=TOTAL_TSPS {
        for j in 0..=(TOTAL_TSPS - i) {
            for k in 0..=(TOTAL_TSPS - i - j) {
                let l = TOTAL_TSPS - i - j - k;
                let score = calculate_score(
                    ings,
                    &[i as isize, j as isize, k as isize, l as isize],
                    count_calories,
                );
                if score > max_score {
                    max_score = score;
                }
            }
        }
    }
    max_score
}
pub fn part1(input: String) -> String {
    let ings = parse_input(&input);
    iterate_tsp_count(&ings, false).to_string()
}

pub fn part2(input: String) -> String {
    let ings = parse_input(&input);
    iterate_tsp_count(&ings, true).to_string()
}
