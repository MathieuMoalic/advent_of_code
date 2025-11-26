pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}
#[derive(Debug)]
struct Deer {
    speed: usize,
    fly_time: usize,
    rest_time: usize,
    distance: usize,
    points: usize,
}
impl Deer {
    fn distance_after_time(&self, time: usize) -> usize {
        let dash_distance = self.speed * self.fly_time;
        let dash_time = self.rest_time + self.fly_time;
        let dash_count = time / dash_time;

        let remainder_time = time % dash_time;
        let last_dash_time = self.fly_time.min(remainder_time);
        let last_dash_distance = last_dash_time * self.speed;

        dash_distance * dash_count + last_dash_distance
    }
}

fn parse_input(input: &str) -> Vec<Deer> {
    let mut deers = Vec::new();
    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let speed = words.get(3).unwrap().parse::<usize>().unwrap();
        let fly_time = words.get(6).unwrap().parse::<usize>().unwrap();
        let rest_time = words.get(13).unwrap().parse::<usize>().unwrap();
        deers.push(Deer {
            speed,
            fly_time,
            rest_time,
            distance: 0,
            points: 0,
        });
    }
    deers
}

pub fn part1(input: String) -> String {
    let race_time = 2503;
    let deers = parse_input(&input);
    let mut furthest_distance = 0;
    for deer in deers {
        furthest_distance = furthest_distance.max(deer.distance_after_time(race_time));
    }
    furthest_distance.to_string()
}
pub fn part2(input: String) -> String {
    let race_time = 2503;
    let mut deers = parse_input(&input);
    for time in 1..=race_time {
        let mut furthest_distance = 0;
        for deer in &mut deers {
            deer.distance = deer.distance_after_time(time);
            furthest_distance = furthest_distance.max(deer.distance);
        }
        for deer in &mut deers {
            if deer.distance == furthest_distance {
                deer.points += 1;
            }
        }
    }
    let mut max_points = 0;
    for deer in deers {
        max_points = max_points.max(deer.points);
    }
    max_points.to_string()
}
