use std::collections::HashMap;

pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}
#[derive(Debug, Clone)]
struct Character {
    hit_points: isize,
    damage: isize,
    armor: isize,
}

fn parse_input(input: String) -> Character {
    let lines = input.lines().collect::<Vec<&str>>();
    Character {
        hit_points: lines[0].split(": ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap(),
        damage: lines[1].split(": ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap(),
        armor: lines[2].split(": ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap(),
    }
}

#[derive(Debug)]
struct Item {
    cost: usize,
    damage: isize,
    armor: isize,
}

type ItemList = HashMap<String, Item>;
fn get_items() -> (ItemList, ItemList, ItemList) {
    let mut weapons = ItemList::new();
    weapons.insert(
        "Dagger".into(),
        Item {
            cost: 8,
            damage: 4,
            armor: 0,
        },
    );
    weapons.insert(
        "Shortsword".into(),
        Item {
            cost: 10,
            damage: 5,
            armor: 0,
        },
    );
    weapons.insert(
        "Warhammer".into(),
        Item {
            cost: 25,
            damage: 6,
            armor: 0,
        },
    );
    weapons.insert(
        "Longsword".into(),
        Item {
            cost: 40,
            damage: 7,
            armor: 0,
        },
    );
    weapons.insert(
        "Greataxe".into(),
        Item {
            cost: 74,
            damage: 8,
            armor: 0,
        },
    );

    let mut armor = ItemList::new();
    armor.insert(
        "None".into(),
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
    ); // optional
    armor.insert(
        "Leather".into(),
        Item {
            cost: 13,
            damage: 0,
            armor: 1,
        },
    );
    armor.insert(
        "Chainmail".into(),
        Item {
            cost: 31,
            damage: 0,
            armor: 2,
        },
    );
    armor.insert(
        "Splintmail".into(),
        Item {
            cost: 53,
            damage: 0,
            armor: 3,
        },
    );
    armor.insert(
        "Bandedmail".into(),
        Item {
            cost: 75,
            damage: 0,
            armor: 4,
        },
    );
    armor.insert(
        "Platemail".into(),
        Item {
            cost: 102,
            damage: 0,
            armor: 5,
        },
    );

    let mut rings = ItemList::new();
    rings.insert(
        "None".into(),
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
    );
    rings.insert(
        "None2".into(),
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
    );
    rings.insert(
        "Damage +1".into(),
        Item {
            cost: 25,
            damage: 1,
            armor: 0,
        },
    );
    rings.insert(
        "Damage +2".into(),
        Item {
            cost: 50,
            damage: 2,
            armor: 0,
        },
    );
    rings.insert(
        "Damage +3".into(),
        Item {
            cost: 100,
            damage: 3,
            armor: 0,
        },
    );
    rings.insert(
        "Defense +1".into(),
        Item {
            cost: 20,
            damage: 0,
            armor: 1,
        },
    );
    rings.insert(
        "Defense +2".into(),
        Item {
            cost: 40,
            damage: 0,
            armor: 2,
        },
    );
    rings.insert(
        "Defense +3".into(),
        Item {
            cost: 80,
            damage: 0,
            armor: 3,
        },
    );

    (weapons, armor, rings)
}

fn turn(player: &mut Character, boss: &mut Character) {
    boss.hit_points -= 1.max(player.damage - boss.armor);
    if boss.hit_points < 1 {
        return;
    }
    player.hit_points -= 1.max(boss.damage - player.armor);
}

fn win_fight(player: &mut Character, boss: &mut Character) -> bool {
    while player.hit_points > 0 && boss.hit_points > 0 {
        turn(player, boss);
    }
    player.hit_points > 0
}

fn ring_pairs(rings: &ItemList) -> Vec<(String, String)> {
    let keys: Vec<String> = rings.keys().cloned().collect();
    let mut pairs = Vec::new();

    for i in 0..keys.len() {
        for j in (i + 1)..keys.len() {
            pairs.push((keys[i].clone(), keys[j].clone()));
        }
    }

    pairs
}

pub fn part1(input: String) -> String {
    let boss_base = parse_input(input);
    let player_base = Character {
        hit_points: 100,
        damage: 0,
        armor: 0,
    };
    let (weapons, armors, rings) = get_items();
    let mut lowest_gold = usize::MAX;
    for weapon in weapons.values() {
        for armor in armors.values() {
            for (ring_name1, ring_name2) in ring_pairs(&rings) {
                let ring1 = rings.get(&ring_name1).unwrap();
                let ring2 = rings.get(&ring_name2).unwrap();
                let gold = ring1.cost + ring2.cost + weapon.cost + armor.cost;

                let mut player = player_base.clone();
                let mut boss = boss_base.clone();

                player.damage += ring1.damage + ring2.damage + weapon.damage;
                player.armor += ring1.armor + ring2.armor + armor.armor;

                if win_fight(&mut player, &mut boss) {
                    lowest_gold = lowest_gold.min(gold);
                }
            }
        }
    }
    lowest_gold.to_string()
}

pub fn part2(input: String) -> String {
    let boss_base = parse_input(input);
    let player_base = Character {
        hit_points: 100,
        damage: 0,
        armor: 0,
    };
    let (weapons, armors, rings) = get_items();
    let mut highest_gold = usize::MIN;
    for weapon in weapons.values() {
        for armor in armors.values() {
            for (ring_name1, ring_name2) in ring_pairs(&rings) {
                let ring1 = rings.get(&ring_name1).unwrap();
                let ring2 = rings.get(&ring_name2).unwrap();
                let gold = ring1.cost + ring2.cost + weapon.cost + armor.cost;

                let mut player = player_base.clone();
                let mut boss = boss_base.clone();

                player.damage += ring1.damage + ring2.damage + weapon.damage;
                player.armor += ring1.armor + ring2.armor + armor.armor;

                if !win_fight(&mut player, &mut boss) {
                    highest_gold = highest_gold.max(gold);
                }
            }
        }
    }
    highest_gold.to_string()
}
