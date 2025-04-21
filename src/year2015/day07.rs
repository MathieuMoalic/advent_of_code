pub fn run(part: usize, input: String) -> String {
    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part: {}", part),
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
enum LogicGate {
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
    INPUT,
}

#[derive(Debug)]
struct Instruction {
    input1: String,
    input2: Option<String>,
    logic_gate: LogicGate,
    output: String,
}
impl Instruction {
    pub fn try_evaluate(&self, wires: &HashMap<String, u16>) -> Option<u16> {
        use LogicGate::*;

        match self.logic_gate {
            INPUT => resolve(wires, &self.input1),
            NOT => {
                let val = resolve(wires, &self.input1)?;
                Some(!val)
            }
            AND => {
                let a = resolve(wires, &self.input1)?;
                let b = self.input2.as_ref().and_then(|s| resolve(wires, s))?;
                Some(a & b)
            }
            OR => {
                let a = resolve(wires, &self.input1)?;
                let b = self.input2.as_ref().and_then(|s| resolve(wires, s))?;
                Some(a | b)
            }
            LSHIFT => {
                let a = resolve(wires, &self.input1)?;
                let b = self.input2.as_ref().and_then(|s| s.parse::<u16>().ok())?;
                Some(a << b)
            }
            RSHIFT => {
                let a = resolve(wires, &self.input1)?;
                let b = self.input2.as_ref().and_then(|s| s.parse::<u16>().ok())?;
                Some(a >> b)
            }
        }
    }
}

fn parse_instructions(input: String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let (input, output) = line.split_once(" -> ").unwrap();
        if !input.contains(" ") {
            instructions.push(Instruction {
                input1: input.to_owned(),
                input2: None,
                logic_gate: LogicGate::INPUT,
                output: output.to_owned(),
            });
        } else if input.contains("NOT") {
            let (_, input1) = input.split_once(" ").unwrap();
            instructions.push(Instruction {
                input1: input1.to_owned(),
                input2: None,
                logic_gate: LogicGate::NOT,
                output: output.to_owned(),
            });
        } else {
            let (input1, rest) = input.split_once(" ").unwrap();
            let (logic_gate_str, input2) = rest.split_once(" ").unwrap();
            let logic_gate: LogicGate = match logic_gate_str {
                "AND" => LogicGate::AND,
                "OR" => LogicGate::OR,
                "LSHIFT" => LogicGate::LSHIFT,
                "RSHIFT" => LogicGate::RSHIFT,
                _ => panic!("Unexpected lg {logic_gate_str} "),
            };
            let value = Instruction {
                input1: input1.to_owned(),
                input2: Some(input2.to_owned()),
                logic_gate,
                output: output.to_owned(),
            };
            instructions.push(value);
        }
    }
    instructions
}

fn resolve(wires: &HashMap<String, u16>, s: &str) -> Option<u16> {
    s.parse::<u16>().ok().or_else(|| wires.get(s).copied())
}

fn run_instruction(mut instructions: Vec<Instruction>) -> HashMap<String, u16> {
    let mut wires: HashMap<String, u16> = HashMap::new();
    while !instructions.is_empty() {
        let mut instr_done: Vec<usize> = Vec::new();
        for (idx, i) in instructions.iter().enumerate() {
            if let Some(result) = i.try_evaluate(&wires) {
                wires.insert(i.output.clone(), result);
                instr_done.push(idx);
            }
        }

        if instr_done.is_empty() {
            break;
        }
        instr_done.reverse();
        for i in instr_done {
            instructions.remove(i);
        }
    }
    wires
}

use std::collections::HashMap;
pub fn part1(input: String) -> String {
    let instructions = parse_instructions(input.to_string());
    let wires = run_instruction(instructions);
    wires.get("a").unwrap().to_string()
}

pub fn part2(input: String) -> String {
    let mut instructions = parse_instructions(input.to_string());
    let wires = run_instruction(instructions);

    // set b to the output of a
    let a = wires.get("a").unwrap().to_string();
    instructions = parse_instructions(input.to_string());
    let mut idx = 0;
    while idx < instructions.len() {
        let i = &mut instructions[idx];
        if i.output == "b" {
            i.input1 = a.clone();
            break;
        }
        idx += 1;
    }

    let wires = run_instruction(instructions);
    wires.get("a").unwrap().to_string()
}
