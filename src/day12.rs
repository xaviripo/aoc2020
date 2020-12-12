pub mod part1;
pub mod part2;

pub const INPUT_FILE: &str = "input/12.txt";

fn manhattan_distance((x, y): (isize, isize)) -> usize {
    (x.abs() + y.abs()) as usize
}

enum Letter {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

struct Instruction {
    letter: Letter,
    argument: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_instruction(line: &String) -> Instruction {
    let mut iter = line.chars().clone();
    let letter: Letter = match iter.next().unwrap() {
        'N' => Letter::N,
        'S' => Letter::S,
        'E' => Letter::E,
        'W' => Letter::W,
        'L' => Letter::L,
        'R' => Letter::R,
        _ => Letter::F,
    };
    let argument: usize = iter.collect::<String>().parse::<usize>().unwrap();
    Instruction {
        letter,
        argument,
    }
}
