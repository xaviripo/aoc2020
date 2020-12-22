use std::collections::VecDeque;

use crate::lib;

use super::parse;

struct Game {
    deck1: VecDeque<usize>,
    deck2: VecDeque<usize>,
}

impl Iterator for Game {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let (deck1, deck2) = next_decks((self.deck1.clone(), self.deck2.clone()))?;
        self.deck1 = deck1;
        self.deck2 = deck2;
        Some(())
    }
}

/// Returns the decks after this round, or None if one of the decks is already empty
fn next_decks((mut deck1, mut deck2): (VecDeque<usize>, VecDeque<usize>)) -> Option<(VecDeque<usize>, VecDeque<usize>)> {
    let n1 = deck1.pop_front()?;
    let n2 = deck2.pop_front()?;
    if n1 > n2 {
        deck1.push_back(n1);
        deck1.push_back(n2);
    } else {
        deck2.push_back(n2);
        deck2.push_back(n1);
    };
    Some((deck1, deck2))
}

fn score((deck1, deck2): (VecDeque<usize>, VecDeque<usize>)) -> usize {

    let mut game = Game { deck1, deck2 };
    while let Some(_) = game.next() {}

    if game.deck1.len() != 0 {
        game.deck1
    } else {
        game.deck2
    }
    .into_iter()
    .rev()
    .enumerate()
    .map(|(index, card)| (index + 1) * card)
    .sum()

}

pub fn run(file: &str) -> std::io::Result<usize> {
    let input = lib::read_lines(file)?;
    Ok(score(parse(input)))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";

    #[test]
    fn run() {
        let input = INPUT.lines().map(|s| s.to_string());
        assert_eq!(306, score(parse(input)))
    }

}
