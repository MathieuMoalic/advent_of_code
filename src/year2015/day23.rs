pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

// hlf r sets register r to half its current value, then continues with the next instruction.
// tpl r sets register r to triple its current value, then continues with the next instruction.
// inc r increments register r, adding 1 to it, then continues with the next instruction.
// jmp offset is a jump; it continues with the instruction offset away relative to itself.
// jie r, offset is like jmp, but only jumps if register r is even ("jump if even").
// jio r, offset is like jmp, but only jumps if register r is 1 ("jump if one", not odd).

#[derive(Clone, Copy, Debug)]
enum InstructionEnum {
    Hlf,
    Tpl,
    Inc,
    Jmp,
    Jie,
    Jio,
}

#[derive(Clone, Copy, Debug)]
enum Reg {
    A,
    B,
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    ins: InstructionEnum,
    jump: Option<i64>,
    reg: Option<Reg>,
}

#[derive(Clone, Debug)]
struct State {
    i: i64,
    a: u64,
    b: u64,
    instructions: Vec<Instruction>,
}

impl State {
    fn reg(&self, r: Reg) -> &u64 {
        match r {
            Reg::A => &self.a,
            Reg::B => &self.b,
        }
    }

    fn reg_mut(&mut self, r: Reg) -> &mut u64 {
        match r {
            Reg::A => &mut self.a,
            Reg::B => &mut self.b,
        }
    }
    fn from_input(input: String) -> State {
        let mut v = Vec::new();
        let input = input.replace(",", "");
        for line in input.lines() {
            let words = line.split_whitespace().collect::<Vec<&str>>();
            let w1 = *words.first().unwrap();
            let w2 = *words.get(1).unwrap();
            let i = match w1 {
                "hlf" => Instruction {
                    ins: InstructionEnum::Hlf,
                    jump: None,
                    reg: Some(match w2 {
                        "a" => Reg::A,
                        "b" => Reg::B,
                        _ => unreachable!("Wrong register"),
                    }),
                },
                "tpl" => Instruction {
                    ins: InstructionEnum::Tpl,
                    jump: None,
                    reg: Some(match w2 {
                        "a" => Reg::A,
                        "b" => Reg::B,
                        _ => unreachable!("Wrong register"),
                    }),
                },
                "inc" => Instruction {
                    ins: InstructionEnum::Inc,
                    jump: None,
                    reg: Some(match w2 {
                        "a" => Reg::A,
                        "b" => Reg::B,
                        _ => unreachable!("Wrong register"),
                    }),
                },
                "jmp" => Instruction {
                    ins: InstructionEnum::Jmp,
                    jump: Some(w2.parse().unwrap()),
                    reg: None,
                },
                "jie" => Instruction {
                    ins: InstructionEnum::Jie,
                    jump: Some(words.get(2).unwrap().parse().unwrap()),
                    reg: Some(match w2 {
                        "a" => Reg::A,
                        "b" => Reg::B,
                        _ => unreachable!("Wrong register"),
                    }),
                },
                "jio" => Instruction {
                    ins: InstructionEnum::Jio,
                    jump: Some(words.get(2).unwrap().parse().unwrap()),
                    reg: Some(match w2 {
                        "a" => Reg::A,
                        "b" => Reg::B,
                        _ => unreachable!("Wrong register"),
                    }),
                },
                _ => unreachable!(),
            };
            v.push(i);
        }
        State {
            i: 0,
            a: 0,
            b: 0,
            instructions: v,
        }
    }

    fn step(&mut self) -> bool {
        if self.i >= self.instructions.len().try_into().unwrap() {
            return false;
        }
        let ins = self.instructions[self.i as usize];
        match ins.ins {
            InstructionEnum::Hlf => {
                if let Some(r) = ins.reg {
                    *self.reg_mut(r) /= 2;
                }
                self.i += 1;
            }
            InstructionEnum::Tpl => {
                if let Some(r) = ins.reg {
                    *self.reg_mut(r) *= 3;
                }
                self.i += 1;
            }
            InstructionEnum::Inc => {
                if let Some(r) = ins.reg {
                    *self.reg_mut(r) += 1;
                }
                self.i += 1;
            }
            InstructionEnum::Jmp => {
                if let Some(j) = ins.jump {
                    self.i += j;
                }
            }
            InstructionEnum::Jie => {
                // only jumps if register is even
                if let Some(r) = ins.reg {
                    if self.reg(r) % 2 == 0 {
                        if let Some(j) = ins.jump {
                            self.i += j;
                        }
                    } else {
                        self.i += 1;
                    }
                }
            }
            InstructionEnum::Jio => {
                if let Some(r) = ins.reg {
                    if *self.reg(r) == 1 {
                        if let Some(j) = ins.jump {
                            self.i += j;
                        }
                    } else {
                        self.i += 1;
                    }
                }
            }
        }
        true
    }
}
fn pprint(s: &State) {
    println!("i={},a={},b={}", s.i, s.a, s.b)
}

pub fn part1(input: String) -> String {
    let mut state = State::from_input(input);
    while state.step() {}
    state.b.to_string()
}
pub fn part2(input: String) -> String {
    let mut state = State::from_input(input);
    state.a = 1;
    while state.step() {}
    state.b.to_string()
}
