pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}
#[derive(Debug, Default)]
struct Sue {
    number: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}
enum Comparison {
    Greater,
    Fewer,
    Equal,
}
impl Sue {
    fn add_item(&mut self, item: &str, quantity: usize) {
        match item {
            "children" => self.children = Some(quantity),
            "cats" => self.cats = Some(quantity),
            "samoyeds" => self.samoyeds = Some(quantity),
            "pomeranians" => self.pomeranians = Some(quantity),
            "akitas" => self.akitas = Some(quantity),
            "vizslas" => self.vizslas = Some(quantity),
            "goldfish" => self.goldfish = Some(quantity),
            "trees" => self.trees = Some(quantity),
            "cars" => self.cars = Some(quantity),
            "perfumes" => self.perfumes = Some(quantity),
            _ => (),
        }
    }

    fn from_line(line: &str) -> Self {
        let mut sue = Self::default();
        let line = line.replace(",", "").replace(":", "");
        let words = line.split_whitespace().collect::<Vec<&str>>();
        sue.number = words[1].parse::<usize>().unwrap();
        sue.add_item(words[2], words[3].parse().unwrap());
        sue.add_item(words[4], words[5].parse().unwrap());
        sue.add_item(words[6], words[7].parse().unwrap());
        sue
    }

    fn cmp_option(a: Option<usize>, b: Option<usize>, comparison: Comparison) -> bool {
        match (a, b) {
            (Some(x), Some(y)) => match comparison {
                Comparison::Greater => x < y,
                Comparison::Fewer => x > y,
                Comparison::Equal => x == y,
            },
            _ => true, // if either is None, we consider it a match
        }
    }
    fn is_match_part1(&self, sue: &Self) -> bool {
        Sue::cmp_option(self.children, sue.children, Comparison::Equal)
            && Sue::cmp_option(self.cats, sue.cats, Comparison::Equal)
            && Sue::cmp_option(self.samoyeds, sue.samoyeds, Comparison::Equal)
            && Sue::cmp_option(self.pomeranians, sue.pomeranians, Comparison::Equal)
            && Sue::cmp_option(self.akitas, sue.akitas, Comparison::Equal)
            && Sue::cmp_option(self.vizslas, sue.vizslas, Comparison::Equal)
            && Sue::cmp_option(self.goldfish, sue.goldfish, Comparison::Equal)
            && Sue::cmp_option(self.trees, sue.trees, Comparison::Equal)
            && Sue::cmp_option(self.cars, sue.cars, Comparison::Equal)
            && Sue::cmp_option(self.perfumes, sue.perfumes, Comparison::Equal)
    }

    fn is_match_part2(&self, sue: &Self) -> bool {
        Sue::cmp_option(self.children, sue.children, Comparison::Equal)
            && Sue::cmp_option(self.cats, sue.cats, Comparison::Greater)
            && Sue::cmp_option(self.samoyeds, sue.samoyeds, Comparison::Equal)
            && Sue::cmp_option(self.pomeranians, sue.pomeranians, Comparison::Fewer)
            && Sue::cmp_option(self.akitas, sue.akitas, Comparison::Equal)
            && Sue::cmp_option(self.vizslas, sue.vizslas, Comparison::Equal)
            && Sue::cmp_option(self.goldfish, sue.goldfish, Comparison::Fewer)
            && Sue::cmp_option(self.trees, sue.trees, Comparison::Greater)
            && Sue::cmp_option(self.cars, sue.cars, Comparison::Equal)
            && Sue::cmp_option(self.perfumes, sue.perfumes, Comparison::Equal)
    }
}

const GIFT: Sue = Sue {
    number: 0,
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1),
};

fn parse_input(input: &str) -> Vec<Sue> {
    input.lines().map(Sue::from_line).collect()
}

pub fn part1(input: String) -> String {
    parse_input(&input)
        .into_iter()
        .find(|sue| GIFT.is_match_part1(sue))
        .map(|s| s.number.to_string())
        .expect("No matches")
}
pub fn part2(input: String) -> String {
    parse_input(&input)
        .into_iter()
        .find(|sue| GIFT.is_match_part2(sue))
        .map(|s| s.number.to_string())
        .expect("No matches")
}
