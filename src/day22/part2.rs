use std::collections::{HashSet, VecDeque};

use crate::lib;

use super::parse;

struct Game {
    decks: (VecDeque<usize>, VecDeque<usize>),
    memory: HashSet<(VecDeque<usize>, VecDeque<usize>)>,
    first_wins: bool,
}

impl Iterator for Game {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.memory.contains(&self.decks) {
            self.first_wins = true;
            return None;
        }

        self.memory.insert(self.decks.clone());

        let (decks, first_wins) = next_decks(self.decks.clone());
        self.first_wins = first_wins;

        if let Some(decks) = decks {
            self.decks = decks;
            Some(())
        } else {
            None
        }

    }
}

/// Returns the decks after this round, or None if the sub-game ends,
/// together with a boolean indicating whether first player wins the round
fn next_decks(mut decks: (VecDeque<usize>, VecDeque<usize>)) -> (Option<(VecDeque<usize>, VecDeque<usize>)>, bool) {

    // Try to get the two cards, otherwise first wins by default
    let (n1, n2) = {
        let n1 = decks.0.pop_front();
        if n1 == None {
            return (None, false);
        }
        let n2 = decks.1.pop_front();
        if n2 == None {
            return (None, true);
        }
        (n1.unwrap(), n2.unwrap())
    };

    // Try to play a sub-game
    let first_wins = if n1 <= decks.0.len() && n2 <= decks.1.len() {
        let mut subgame = Game { decks: decks.clone(), first_wins: false, memory: HashSet::new() };
        subgame.decks.0 = subgame.decks.0.into_iter().take(n1).collect();
        subgame.decks.1 = subgame.decks.1.into_iter().take(n2).collect();
        while let Some(_) = subgame.next() {}
        subgame.first_wins
    } else {
        // Can't play sub-game so we resort to common Combat rules
        n1 > n2
    };

    if first_wins {
        decks.0.push_back(n1);
        decks.0.push_back(n2);
    } else {
        decks.1.push_back(n2);
        decks.1.push_back(n1);
    };
    (Some(decks), first_wins)

}

fn score(decks: (VecDeque<usize>, VecDeque<usize>)) -> usize {

    let mut game = Game { decks, first_wins: false, memory: HashSet::new() };
    while let Some(_) = game.next() {}

    if game.first_wins { game.decks.0 } else { game.decks.1 }
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
        assert_eq!(291, score(parse(input)))
    }

}
