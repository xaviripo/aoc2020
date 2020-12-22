pub mod part1;
pub mod part2;

use std::collections::VecDeque;

pub const INPUT_FILE: &str = "input/22.txt";

fn parse<T: Iterator<Item=String>>(mut lines: T) -> (VecDeque<usize>, VecDeque<usize>) {

    let deck1: VecDeque<usize> = lines
    .by_ref()
    .take_while(|line| line != "")
    .skip(1)
    .map(|line| line.parse().unwrap())
    .collect();

    let deck2: VecDeque<usize> = lines
    .take_while(|line| line != "")
    .skip(1)
    .map(|line| line.parse().unwrap())
    .collect();

    (deck1, deck2)

}
