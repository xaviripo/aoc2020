use std::collections::HashMap;

use crate::lib;

pub const INPUT_FILE: &str = "input/15.txt";

fn parse(input: String) -> Vec<usize> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Game {
    /// Maps each number to the last position it appeared in
    memory: HashMap<usize, usize>,
    /// Last position and number, not yet in memory
    position: usize,
    number: usize,
}

impl Game {
    fn from_starting(starting: Vec<usize>) -> impl Iterator<Item=usize> {
        let number = *starting.last().unwrap();
        let position = starting.len() - 1;
        let game = Game {
            memory: starting[..position].into_iter().enumerate().fold(HashMap::new(), |mut memory, (position, number)| {
                memory.insert(*number, position);
                memory
            }),
            number,
            position,
        };

        starting.into_iter().chain(game)

    }
}

impl Iterator for Game {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {

        let next = if let Some(prev_position) = self.memory.get(&self.number) {
            self.position - prev_position
        } else {
            0
        };

        self.memory.insert(self.number, self.position);

        self.position += 1;
        self.number = next;

        Some(self.number)

    }
}

fn get_2020(mut sequence: impl Iterator<Item=usize>) -> usize {
    sequence.nth(2020 - 1).unwrap()
}

fn get_30000000(mut sequence: impl Iterator<Item=usize>) -> usize {
    sequence.nth(30_000_000 - 1).unwrap()
}

pub fn run1(file: &str) -> std::io::Result<usize> {
    let line = lib::read_lines(file)?.next().unwrap();
    Ok(get_2020(Game::from_starting(parse(line))))
}

pub fn run2(file: &str) -> std::io::Result<usize> {
    let line = lib::read_lines(file)?.next().unwrap();
    Ok(get_30000000(Game::from_starting(parse(line))))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run1() {
        let cases = [
            ("0,3,6", 436),
            ("1,3,2", 1),
            ("2,1,3", 10),
            ("1,2,3", 27),
            ("2,3,1", 78),
            ("3,2,1", 438),
            ("3,1,2", 1836),
        ];
        for &(input, output) in cases.iter() {
            assert_eq!(output, get_2020(Game::from_starting(parse(input.to_string()))));
        }
    }

    #[test]
    fn run2() {
        let cases = [
            ("0,3,6", 175594),
            ("1,3,2", 2578),
            ("2,1,3", 3544142),
            ("1,2,3", 261214),
            ("2,3,1", 6895259),
            ("3,2,1", 18),
            ("3,1,2", 362),
        ];
        for &(input, output) in cases.iter() {
            assert_eq!(output, get_30000000(Game::from_starting(parse(input.to_string()))));
        }
    }

}
