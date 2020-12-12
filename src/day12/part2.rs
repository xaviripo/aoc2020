use crate::lib;

use super::{Letter, manhattan_distance, parse_instruction};

#[derive(Debug, Clone, Copy)]
struct Ship {
    position: (isize, isize),
    /// The waypoint's coordinates are relative to the ship
    waypoint: (isize, isize),
}

fn rotate_r(mut ship: Ship, times: usize) -> Ship {
    for _ in 0..(times % 4) {
        ship.waypoint = (ship.waypoint.1, -ship.waypoint.0);
    }
    ship
}

fn rotate_l(ship: Ship, times: usize) -> Ship {
    // Modular arithmetic affords me not to think
    rotate_r(ship, 4 - (times % 4))
}

fn destination(instructions: Vec<String>) -> (isize, isize) {

    let ship = Ship {
        position: (0, 0),
        waypoint: (10, 1),
    };

    instructions.iter()
    .map(|line| parse_instruction(&line))
    .fold(ship, |mut ship, instruction| {
        match instruction.letter {
            Letter::N => {
                ship.waypoint = (ship.waypoint.0, ship.waypoint.1 + instruction.argument as isize);
            },
            Letter::S => {
                ship.waypoint = (ship.waypoint.0, ship.waypoint.1 - instruction.argument as isize);
            },
            Letter::E => {
                ship.waypoint = (ship.waypoint.0 + instruction.argument as isize, ship.waypoint.1);
            },
            Letter::W => {
                ship.waypoint = (ship.waypoint.0 - instruction.argument as isize, ship.waypoint.1);
            },
            Letter::R => {
                let arg = (instruction.argument % 360) / 90;
                ship = rotate_r(ship, arg);
            },
            Letter::L => {
                let arg = (instruction.argument % 360) / 90;
                ship = rotate_l(ship, arg);
            }
            Letter::F => {
                for _ in 0..instruction.argument {
                    ship.position = (ship.position.0 + ship.waypoint.0, ship.position.1 + ship.waypoint.1);
                }
            }
        };
        ship
    }).position

}

pub fn run(file: &str) -> std::io::Result<usize> {
    let instructions: Vec<String> = lib::read_lines(file)?.collect();
    Ok(manhattan_distance(destination(instructions)))
}

#[cfg(test)]
mod test {
    use super::{destination, manhattan_distance};

    const INPUT: &str =
"F10
N3
F7
R90
F11";

    #[test]
    fn run() {
        let instructions: Vec<String> = INPUT.lines().map(|s| s.to_string()).collect();
        assert_eq!(286, manhattan_distance(destination(instructions)));
    }

}
