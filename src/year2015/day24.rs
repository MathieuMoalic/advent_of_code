pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

fn parse_input(input: String) -> Vec<u16> {
    let mut v: Vec<u16> = input.lines().map(|x| x.parse().unwrap()).collect();
    // heuristic: big items first helps pruning
    v.sort_unstable_by(|a, b| b.cmp(a));
    v
}

fn qe(v: &[u16]) -> u128 {
    v.iter().map(|&x| x as u128).product::<u128>()
}

// return true if `cand` is better than `best` w.r.t. group 0
fn better(best: &[Vec<u16>; 3], cand: &[Vec<u16>; 3]) -> bool {
    if cand[0].len() < best[0].len() {
        return true;
    }
    if cand[0].len() == best[0].len() && qe(&cand[0]) < qe(&best[0]) {
        return true;
    }
    false
}
fn better2(best: &[Vec<u16>; 4], cand: &[Vec<u16>; 4]) -> bool {
    if cand[0].len() < best[0].len() {
        return true;
    }
    if cand[0].len() == best[0].len() && qe(&cand[0]) < qe(&best[0]) {
        return true;
    }
    false
}

fn rec(
    pkgs: &[u16],
    idx: usize,
    stacks: &mut [Vec<u16>; 3],
    sums: &mut [u16; 3],
    target: u16,
    best: &mut Option<[Vec<u16>; 3]>,
) {
    // prune if any stack is already overweight
    if sums.iter().any(|&s| s > target) {
        return;
    }

    // optional pruning: if we already have a best, don't grow group 0 beyond it
    if let Some(b) = best {
        if stacks[0].len() > b[0].len() {
            return;
        }
        if stacks[0].len() == b[0].len() && qe(&stacks[0]) >= qe(&b[0]) {
            // only safe when all three sums can't shrink; still good as a weak bound
            // because qe increases when we multiply by positive integers
            // (we only ever *add* items to group 0)
            return;
        }
    }

    if idx == pkgs.len() {
        // all items placed: check perfect split
        if sums.iter().all(|&s| s == target) {
            let cand = stacks.clone();
            match best {
                None => *best = Some(cand),
                Some(b) => {
                    if better(b, &cand) {
                        *best = Some(cand);
                    }
                }
            }
        }
        return;
    }

    let w = pkgs[idx];

    // try placing current package into each stack
    for s in 0..3 {
        stacks[s].push(w);
        sums[s] += w;

        rec(pkgs, idx + 1, stacks, sums, target, best);

        sums[s] -= w;
        stacks[s].pop();

        // symmetry cut: don't place into later empty stacks if an earlier one is empty
        // (prevents permutations of identical empty stacks)
        if stacks[s].is_empty() {
            break;
        }
    }
}

pub fn part1(input: String) -> String {
    let pkgs = parse_input(input);
    let total: u16 = pkgs.iter().copied().sum();
    let target = total / 3;

    // symmetry break: force the first (largest after sort) package into group 0
    let mut stacks: [Vec<u16>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut sums: [u16; 3] = [0, 0, 0];

    if pkgs.is_empty() || total % 3 != 0 {
        return "no solution".to_string();
    }

    stacks[0].push(pkgs[0]);
    sums[0] += pkgs[0];

    let mut best: Option<[Vec<u16>; 3]> = None;
    rec(&pkgs, 1, &mut stacks, &mut sums, target, &mut best);

    match best {
        Some(b) => qe(&b[0]).to_string(),
        None => "no solution".to_string(),
    }
}

fn rec2(
    pkgs: &[u16],
    idx: usize,
    stacks: &mut [Vec<u16>; 4],
    sums: &mut [u16; 4],
    target: u16,
    best: &mut Option<[Vec<u16>; 4]>,
) {
    // prune if any stack is already overweight
    if sums.iter().any(|&s| s > target) {
        return;
    }

    // optional pruning: if we already have a best, don't grow group 0 beyond it
    if let Some(b) = best {
        if stacks[0].len() > b[0].len() {
            return;
        }
        if stacks[0].len() == b[0].len() && qe(&stacks[0]) >= qe(&b[0]) {
            // only safe when all three sums can't shrink; still good as a weak bound
            // because qe increases when we multiply by positive integers
            // (we only ever *add* items to group 0)
            return;
        }
    }

    if idx == pkgs.len() {
        // all items placed: check perfect split
        if sums.iter().all(|&s| s == target) {
            let cand = stacks.clone();
            match best {
                None => *best = Some(cand),
                Some(b) => {
                    if better2(b, &cand) {
                        *best = Some(cand);
                    }
                }
            }
        }
        return;
    }

    let w = pkgs[idx];

    // try placing current package into each stack
    for s in 0..4 {
        stacks[s].push(w);
        sums[s] += w;

        rec2(pkgs, idx + 1, stacks, sums, target, best);

        sums[s] -= w;
        stacks[s].pop();

        // symmetry cut: don't place into later empty stacks if an earlier one is empty
        // (prevents permutations of identical empty stacks)
        if stacks[s].is_empty() {
            break;
        }
    }
}

pub fn part2(input: String) -> String {
    let pkgs = parse_input(input);
    let total: u16 = pkgs.iter().copied().sum();
    let target = total / 4;

    // symmetry break: force the first (largest after sort) package into group 0
    let mut stacks: [Vec<u16>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut sums: [u16; 4] = [0, 0, 0, 0];

    if pkgs.is_empty() || total % 4 != 0 {
        return "no solution".to_string();
    }

    stacks[0].push(pkgs[0]);
    sums[0] += pkgs[0];

    let mut best: Option<[Vec<u16>; 4]> = None;
    rec2(&pkgs, 1, &mut stacks, &mut sums, target, &mut best);

    match best {
        Some(b) => qe(&b[0]).to_string(),
        None => "no solution".to_string(),
    }
}
