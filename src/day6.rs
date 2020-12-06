use std::collections::BTreeSet;

use crate::lib;

pub const INPUT_FILE: &str = "input/6.txt";

fn questions_any(lines: &[String]) -> usize {
    lines.into_iter()
    .fold(BTreeSet::new(), |mut set, line| {
        line.chars().for_each(|question| { set.insert(question); });
        set
    })
    .len()
}

fn questions_all(lines: &[String]) -> usize {
    let mut sets = lines.into_iter()
    .map(|line| {
        let mut set = BTreeSet::new();
        line.chars().for_each(|question| { set.insert(question); });
        set
    });
    // We want to intersect all sets.
    if let Some(init) = sets.next() {
        sets
        .fold(init, |set1, set2| set1.intersection(&set2).cloned().collect::<BTreeSet<char>>())
        .len()
    } else {
        0
    }
}

fn sum_questions(lines: Vec<String>, any: bool) -> usize {
    lines
    .split(|line| line == "")
    .map(|lines| if any { questions_any(lines) } else { questions_all(lines) })
    .sum()
}

pub fn run1(file: &str) -> std::io::Result<usize> {
    Ok(sum_questions(lib::read_lines(file)?.collect(), true))
}

pub fn run2(file: &str) -> std::io::Result<usize> {
    Ok(sum_questions(lib::read_lines(file)?.collect(), false))
}

#[cfg(test)]
mod test {
    use super::sum_questions;


    const INPUT: &str =
"abc

a
b
c

ab
ac

a
a
a
a

b
";

    #[test]
    fn run1() {
        assert_eq!(11, sum_questions(INPUT.to_string().lines().map(|line| line.to_string()).collect(), true))
    }

    #[test]
    fn run2() {
        assert_eq!(6, sum_questions(INPUT.to_string().lines().map(|line| line.to_string()).collect(), false))
    }

}
