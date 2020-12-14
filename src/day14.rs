use std::collections::BTreeMap;

use crate::lib;

pub const INPUT_FILE: &str = "input/14.txt";

#[derive(Debug)]
enum Instruction {
    SetMask {
        mask: String
    },
    Write {
        address: String,
        value: String,
    }
}

struct ProgramState {
    mask: String,
    memory: BTreeMap<String, String>,
}

fn parse_line(line: String) -> Option<Instruction> {
    if line.starts_with("mask = ") {
        Some(Instruction::SetMask {
            mask: line[7..].to_string()
        })
    } else {
        lazy_static! {
            static ref WRITE_REGEX: regex::Regex = regex::Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
        }
        let captures = WRITE_REGEX.captures_iter(&line).next()?;
        Some(Instruction::Write {
            address: format!("{:036b}", captures[1].parse::<usize>().ok()?),
            value: format!("{:036b}", captures[2].parse::<usize>().ok()?),
        })
    }
}

fn parse_integer(integer: &str) -> Option<usize> {
    usize::from_str_radix(integer, 2).ok()
}

fn apply_mask_v1(value: String, mask: String) -> String {
    value.chars().zip(mask.chars()).map(|(v, m)| match m {
        '0' => '0',
        '1' => '1',
        _ => v,
    })
    .collect::<String>()
}

fn apply_v1(state: ProgramState, instruction: Instruction) -> ProgramState {
    match instruction {
        Instruction::SetMask { mask } => ProgramState { mask, ..state },
        Instruction::Write { address, value } => {
            let mut new_state = ProgramState { ..state };
            new_state.memory.insert(address, apply_mask_v1(value, new_state.mask.clone()));
            new_state
        },
    }
}

fn apply_v2(state: ProgramState, instruction: Instruction) -> ProgramState {
    match instruction {
        Instruction::SetMask { mask } => ProgramState { mask, ..state },
        Instruction::Write { address, value } => {
            let mut new_state = ProgramState { ..state };

            let addresses: Vec<String> = new_state.mask.chars().enumerate().fold(vec![address], |addresses, (i, m)| {
                match m {
                    '1' => addresses.into_iter().map(|address| {
                        let mut v: Vec<char> = address.chars().collect();
                        v[i] = '1';
                        v.into_iter().collect()
                    }).collect(),
                    'X' => {
                        let mut new_addresses: Vec<String> = vec![];
                        for address in addresses {
                            let mut a: Vec<char> = address.chars().collect();
                            let mut b: Vec<char> = a.clone();
                            a[i] = '0'; b[i] = '1';
                            new_addresses.push(a.into_iter().collect());
                            new_addresses.push(b.into_iter().collect());
                        }
                        new_addresses
                    },
                    _ => addresses,
                }
            });

            for address in addresses {
                new_state.memory.insert(address, value.clone());
            }

            new_state
        },
    }
}

fn run(lines: impl Iterator<Item=String>, version2: bool) -> Option<usize> {
    let state = ProgramState {
        mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
        memory: BTreeMap::new(),
    };

    lines
    .map(parse_line)
    .try_fold(state, |state, instruction| Some(if version2 {
        apply_v2(state, instruction?)
    } else {
        apply_v1(state, instruction?)
    }))?
    .memory
    .values()
    .try_fold(0, |acc, cur| Some(acc + parse_integer(cur)?))
}

pub fn run1(file: &str) -> Option<usize> {
    run(lib::read_lines(file).ok()?, false)
}

pub fn run2(file: &str) -> Option<usize> {
    run(lib::read_lines(file).ok()?, true)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str =
"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn run1() {
        assert_eq!(Some(165), run(INPUT_1.lines().map(|s| s.to_string()), false));
    }

    const INPUT_2: &str =
"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn run2() {
        assert_eq!(Some(208), run(INPUT_2.lines().map(|s| s.to_string()), true));
    }

}
