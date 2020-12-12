use crate::lib;

use super::{Direction, Letter, manhattan_distance, parse_instruction};

#[derive(Debug, Clone, Copy)]
struct Ship {
    position: (isize, isize),
    direction: Direction,
}

fn rotate_r(direction: Direction, times: usize) -> Direction {
    use Direction::*;
    let mut result = direction;
    for _ in 0..(times % 4) {
        result = match result {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    result
}

fn rotate_l(direction: Direction, times: usize) -> Direction {
    use Direction::*;
    let mut result = direction;
    for _ in 0..(times % 4) {
        result = match result {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
    result
}

fn destination(instructions: Vec<String>) -> (isize, isize) {

    let ship = Ship {
        position: (0, 0),
        direction: Direction::East,
    };

    instructions.iter()
    .map(|line| parse_instruction(&line))
    .fold(ship, |ship, instruction| {
        let mut new_ship = ship.clone();
        match instruction.letter {
            Letter::N => {
                new_ship.position = (new_ship.position.0, new_ship.position.1 + instruction.argument as isize);
            },
            Letter::S => {
                new_ship.position = (new_ship.position.0, new_ship.position.1 - instruction.argument as isize);
            },
            Letter::E => {
                new_ship.position = (new_ship.position.0 + instruction.argument as isize, new_ship.position.1);
            },
            Letter::W => {
                new_ship.position = (new_ship.position.0 - instruction.argument as isize, new_ship.position.1);
            },
            Letter::R => {
                let arg = (instruction.argument % 360) / 90;
                new_ship.direction = rotate_r(new_ship.direction, arg);
            },
            Letter::L => {
                let arg = (instruction.argument % 360) / 90;
                new_ship.direction = rotate_l(new_ship.direction, arg);
            }
            Letter::F => {
                new_ship.position = match new_ship.direction {
                    Direction::North => (new_ship.position.0, new_ship.position.1 + instruction.argument as isize),
                    Direction::East => (new_ship.position.0 + instruction.argument as isize, new_ship.position.1),
                    Direction::South => (new_ship.position.0, new_ship.position.1 - instruction.argument as isize),
                    Direction::West => (new_ship.position.0 - instruction.argument as isize, new_ship.position.1),
                };
            }
        };
        new_ship
    }).position

}

pub fn run(file: &str) -> std::io::Result<usize> {
    let instructions: Vec<String> = lib::read_lines(file)?.collect();
    Ok(manhattan_distance(destination(instructions)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rotate_r_1_4() {
        assert_eq!(Direction::East, rotate_r(Direction::North, 1));
        assert_eq!(Direction::South, rotate_r(Direction::North, 2));
        assert_eq!(Direction::West, rotate_r(Direction::North, 3));
        assert_eq!(Direction::North, rotate_r(Direction::North, 4));
        assert_eq!(Direction::East, rotate_r(Direction::North, 4001));
    }

    const INPUT_1: &str =
"F10
N3
F7
R90
F11";

    #[test]
    fn run() {
        let instructions: Vec<String> = INPUT_1.lines().map(|s| s.to_string()).collect();
        assert_eq!(25, manhattan_distance(destination(instructions)));
    }

}
