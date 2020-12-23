use std::collections::HashMap;

use crate::lib;

pub const INPUT_FILE: &str = "input/23.txt";

#[derive(Debug)]
struct Game {
    cups: HashMap<usize, usize>,
    current: usize,
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.cups[&self.current];
        self.current = next;
        Some(next)
    }
}

fn next(mut game: Game) -> Game {

    let picked = {
        let picked1 = game.cups[&game.current];
        let picked2 = game.cups[&picked1];
        let picked3 = game.cups[&picked2];
        vec![picked1, picked2, picked3]
    };

    let destination = {
        let mut destination = game.current;
        loop {
            destination = if destination == 1 { game.cups.len() } else { destination - 1 };
            if !picked.contains(&destination) {
                break;
            }
        }
        destination
    };

    game.cups.insert(game.current, game.cups[&picked[2]]);
    game.cups.insert(picked[2], game.cups[&destination]);
    game.cups.insert(destination, picked[0]);

    game.current = game.cups[&game.current];

    game

}

fn play1(input: String, moves: usize) -> String {
    let cups: Vec<usize> = input.chars().map(|c| c.to_string().parse().unwrap()).collect();
    let current = cups[0];
    let cups = cups.clone().into_iter().zip(cups.into_iter().cycle().skip(1)).collect();
    let mut game = Game { cups, current };
    for _ in 0..moves {
        game = next(game);
    }

    game.current = 1;

    game
    .take_while(|n| *n != 1)
    .map(|n| n.to_string())
    .collect()
}

fn play2(input: String, moves: usize) -> usize {
    let cups: Vec<usize> = input.chars().map(|c| c.to_string().parse().unwrap()).chain(10..=1_000_000).collect();
    let current = cups[0];
    let cups = cups.clone().into_iter().zip(cups.into_iter().cycle().skip(1)).collect();
    let mut game = Game { cups, current };
    for _ in 0..moves {
        game = next(game);
    }

    game.current = 1;

    game
    .take(2)
    .product()
}

pub fn run1(file: &str) -> std::io::Result<String> {
    let input = lib::read_lines(file)?.next().unwrap();
    Ok(play1(input, 100))
}

pub fn run2(file: &str) -> std::io::Result<usize> {
    let input = lib::read_lines(file)?.next().unwrap();
    Ok(play2(input, 10_000_000))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str = "389125467";

    #[test]
    fn run1_10() {
        let input = INPUT_1.lines().map(|s| s.to_string()).next().unwrap();
        assert_eq!("92658374", play1(input, 10));
    }

    #[test]
    fn run1_100() {
        let input = INPUT_1.lines().map(|s| s.to_string()).next().unwrap();
        assert_eq!("67384529", play1(input, 100));
    }

    #[test]
    fn run2() {
        let input = INPUT_1.lines().map(|s| s.to_string()).next().unwrap();
        assert_eq!(149245887792, play2(input, 10_000_000));
    }

}
