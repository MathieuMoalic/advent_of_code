pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}
#[derive(Clone, Debug)]
struct Character {
    hp: i32,
    mana: i32,
    damage: i32,
    armor: i32,
}

#[derive(Clone, Debug)]
struct GameState {
    player: Character,
    boss: Character,
    effects: Vec<ActiveEffect>,
    mana_spent: usize,
    part2: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EffectKind {
    Shield,
    Poison,
    Recharge,
}

#[derive(Clone, Copy, Debug)]
struct ActiveEffect {
    kind: EffectKind,
    timer: u8,
}

impl EffectKind {
    fn starting_timer(self) -> u8 {
        match self {
            EffectKind::Shield => 6,
            EffectKind::Poison => 6,
            EffectKind::Recharge => 5,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}
impl Spell {
    /// Iterate all spell variants
    pub fn all() -> &'static [Spell] {
        &[
            Spell::MagicMissile,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge,
        ]
    }
}

fn parse_input(input: String) -> Character {
    let lines = input.lines().collect::<Vec<&str>>();
    Character {
        hp: lines[0].split(": ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap(),
        damage: lines[1].split(": ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap(),
        armor: 0,
        mana: 0,
    }
}

fn apply_effects(state: &mut GameState) {
    state.player.armor = 0;

    for eff in &mut state.effects {
        match eff.kind {
            EffectKind::Shield => {
                state.player.armor = 7;
            }
            EffectKind::Poison => {
                state.boss.hp -= 3;
            }
            EffectKind::Recharge => {
                state.player.mana += 101;
            }
        }

        eff.timer -= 1;
    }

    state.effects.retain(|e| e.timer > 0);
}

fn cast_spell(state: &mut GameState, spell: Spell) -> bool {
    let cost = spell.cost();
    if state.player.mana < cost {
        return false;
    }

    // Can't cast an effect if it's already active.
    let effect_is_active = |kind: EffectKind, effects: &Vec<ActiveEffect>| {
        effects.iter().any(|e| e.kind == kind && e.timer > 0)
    };

    match spell {
        Spell::Shield if effect_is_active(EffectKind::Shield, &state.effects) => return false,
        Spell::Poison if effect_is_active(EffectKind::Poison, &state.effects) => return false,
        Spell::Recharge if effect_is_active(EffectKind::Recharge, &state.effects) => return false,
        _ => {}
    }

    // Pay mana
    state.player.mana -= cost;
    state.mana_spent += cost as usize;

    // Immediate effect or start a timed effect
    match spell {
        Spell::MagicMissile => {
            state.boss.hp -= 4;
        }
        Spell::Drain => {
            state.boss.hp -= 2;
            state.player.hp += 2;
        }
        Spell::Shield => {
            state.effects.push(ActiveEffect {
                kind: EffectKind::Shield,
                timer: EffectKind::Shield.starting_timer(),
            });
        }
        Spell::Poison => {
            state.effects.push(ActiveEffect {
                kind: EffectKind::Poison,
                timer: EffectKind::Poison.starting_timer(),
            });
        }
        Spell::Recharge => {
            state.effects.push(ActiveEffect {
                kind: EffectKind::Recharge,
                timer: EffectKind::Recharge.starting_timer(),
            });
        }
    }

    true
}

fn play_round(mut state: GameState, spell: Spell) -> Option<GameState> {
    // ===== PLAYER TURN =====
    apply_effects(&mut state);
    if state.boss.hp <= 0 {
        // Boss died from effects before player acts.
        return Some(state);
    }

    if state.part2 {
        state.player.hp -= 1;
    }

    if !cast_spell(&mut state, spell) {
        // Illegal move: not enough mana or effect already active.
        return None;
    }

    if state.boss.hp <= 0 {
        // Boss died from the spell.
        return Some(state);
    }

    // ===== BOSS TURN =====
    apply_effects(&mut state);
    if state.boss.hp <= 0 {
        // Boss died from effects before attacking.
        return Some(state);
    }

    // Boss attacks.
    let damage = (state.boss.damage - state.player.armor).max(1);
    state.player.hp -= damage;

    if state.player.hp <= 0 {
        None
    } else {
        Some(state)
    }
}

fn rec(state: &mut Option<GameState>, lowest_mana: &mut usize) {
    if let Some(s) = state {
        if s.boss.hp <= 0 {
            *lowest_mana = s.mana_spent.min(*lowest_mana);
            return;
        }
        if s.mana_spent >= *lowest_mana {
            return;
        }
        for spell in Spell::all() {
            let mut new_state = play_round(s.clone(), *spell);
            rec(&mut new_state, lowest_mana);
        }
    }
}

fn init_state(input: String, part2: bool) -> GameState {
    let boss = parse_input(input);
    let player = Character {
        hp: 50,
        damage: 0,
        armor: 0,
        mana: 500,
    };
    GameState {
        player,
        boss,
        effects: Vec::new(),
        mana_spent: 0,
        part2,
    }
}

pub fn part1(input: String) -> String {
    let state = init_state(input, false);
    let mut lowest_mana: usize = usize::MAX;
    rec(&mut Some(state), &mut lowest_mana);
    lowest_mana.to_string()
}

pub fn part2(input: String) -> String {
    let state = init_state(input, true);
    let mut lowest_mana: usize = usize::MAX;
    rec(&mut Some(state), &mut lowest_mana);
    lowest_mana.to_string()
}
