use std::collections::BTreeSet;

use crate::lib;

pub const INPUT_FILE: &str = "input/8.txt";

#[derive(PartialEq, Eq, Clone, Copy)]
enum InstructionKind {
    Nop,
    Acc,
    Jmp,
}

#[derive(Clone, Copy)]
struct Instruction {
    kind: InstructionKind,
    argument: isize,
}

/// Calculate the value of the accumulator right before executing any instruction for the second time or terminating.
/// Returns (isize, usize), where
/// - isize is the accumulator upon finishing
/// - usize is the line that was about to be executed
fn calculate_acc(instructions: &Vec<Instruction>) -> (isize, usize) {
    let mut visited: BTreeSet<usize> = BTreeSet::new();
    let mut accumulator: isize = 0;
    let mut current: usize = 0;
    loop {

        // Have we been here before?
        if visited.contains(&current) {
            return (accumulator, current);
        }
        visited.insert(current);

        // Inscrease/Decrease accumulator
        accumulator += match instructions[current].kind {
            InstructionKind::Acc => instructions[current].argument,
            _ => 0,
        };

        // Find next instruction
        current = (current as isize + match instructions[current].kind {
            InstructionKind::Jmp => instructions[current].argument,
            _ => 1,
        }) as usize;

        // Exit if we're out of bounds
        if current >= instructions.len() {
            return (accumulator, current);
        }

    }
}

/// Get a set of the visited lines
fn obtain_visited(instructions: &Vec<Instruction>) -> Vec<usize> {
    let mut visited: Vec<usize> = vec![];
    let mut current: usize = 0;
    loop {

        // Have we been here before?
        if visited.contains(&current) {
            return visited;
        }
        visited.push(current);

        // Find next instruction
        current = (current as isize + match instructions[current].kind {
            InstructionKind::Jmp => instructions[current].argument,
            _ => 1,
        }) as usize;

    }
}

fn calculate_acc_terminate(instructions: &Vec<Instruction>) -> Option<isize> {
    let nops_and_jmps = obtain_visited(&instructions)
    .into_iter()
    .filter(|&line| instructions[line].kind != InstructionKind::Acc);

    for instruction in nops_and_jmps {
        let mut cloned_instructions = instructions.clone();
        cloned_instructions[instruction] = match cloned_instructions[instruction].kind {
            InstructionKind::Acc => cloned_instructions[instruction],
            InstructionKind::Nop => Instruction {
                kind: InstructionKind::Jmp,
                argument: cloned_instructions[instruction].argument,
            },
            InstructionKind::Jmp => Instruction {
                kind: InstructionKind::Nop,
                argument: cloned_instructions[instruction].argument,
            },
        };

        let (acc, cur) = calculate_acc(&cloned_instructions);

        if cur == cloned_instructions.len() {
            return Some(acc);
        }
    }

    None

}

fn parse_instructions(lines: impl Iterator<Item=String>) -> impl Iterator<Item=Instruction> {
    lines.map(|line| {
        let mut split = line.split(" ");
        let name = split.next().unwrap();
        let argument = split.next().unwrap().parse::<isize>().unwrap();
        Instruction {
            kind: match name {
                "nop" => InstructionKind::Nop,
                "acc" => InstructionKind::Acc,
                "jmp" => InstructionKind::Jmp,
                // This should not happen
                // The appropriate thing would be to wrap the return type in an Option
                _ => InstructionKind::Nop
            },
            argument
        }
    })
}

pub fn run1(file: &str) -> std::io::Result<isize> {
    Ok(calculate_acc(&parse_instructions(lib::read_lines(file)?).collect()).0)
}

pub fn run2(file: &str) -> Option<isize> {
    calculate_acc_terminate(&parse_instructions(lib::read_lines(file).ok()?).collect())
}

#[cfg(test)]
mod test {
    use super::{calculate_acc, calculate_acc_terminate, parse_instructions};


    const INPUT: &str =
"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

    #[test]
    fn run1() {
        assert_eq!(5, calculate_acc(&parse_instructions(INPUT.lines().map(|s| s.to_string())).collect()).0);
    }

    #[test]
    fn run2() {
        assert_eq!(Some(8), calculate_acc_terminate(&parse_instructions(INPUT.lines().map(|s| s.to_string())).collect()));
    }

}
