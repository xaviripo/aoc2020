use std::collections::VecDeque;

use crate::lib;

pub const INPUT_FILE: &str = "input/9.txt";

fn find_first_wrong(numbers: impl Iterator<Item=u64>, preamble_length: usize) -> Option<(usize, u64)> {

    let mut preamble: VecDeque<u64> = VecDeque::new();

    numbers
    .enumerate()
    .find(|&(_, number)| {

        // Fill the preamble
        if preamble.len() < preamble_length {
            preamble.push_back(number);
            return false;
        }

        let mut i = 1;
        let slice = preamble.make_contiguous();
        for first in &slice[..] {
            for second in &slice[i..] {
                if first != second && first + second == number {
                    preamble.pop_front();
                    preamble.push_back(number);
                    return false;
                }
            }
            i += 1;
        }

        true

    })

}

fn find_contiguous_summands(target: u64, numbers: Vec<u64>) -> Option<u64> {
    for start in 0..numbers.len() {
        for end in start..numbers.len() {
            let range = numbers[start..=end].to_vec();
            if range.iter().sum::<u64>() == target {
                return Some(range.iter().min().unwrap() + range.iter().max().unwrap());
            }
        }
    }
    None
}

fn process2(numbers: Vec<u64>, preamble_length: usize) -> Option<u64> {
    let (index, number) = find_first_wrong(numbers.clone().into_iter(), preamble_length)?;
    let mut before = numbers.clone();
    let after = before.split_off(index);
    find_contiguous_summands(number, before)
    .or(find_contiguous_summands(number, after))
}

pub fn run1(file: &str) -> Option<u64> {
    let numbers = lib::read_lines(file).ok()?.map(|s| s.parse().unwrap());
    Some(find_first_wrong(numbers, 25)?.1)
}

pub fn run2(file: &str) -> Option<u64> {
    let numbers: Vec<u64> = lib::read_lines(file).ok()?.map(|s| s.parse().unwrap()).collect();
    process2(numbers, 25)
}

#[cfg(test)]
mod test {
    use super::{find_first_wrong, process2};


    const INPUT: &str =
"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn run1() {
        assert_eq!(127, find_first_wrong(INPUT.lines().map(|s| s.parse().unwrap()), 5).unwrap().1);
    }

    #[test]
    fn run2() {
        assert_eq!(62, process2(INPUT.lines().map(|s| s.parse().unwrap()).collect(), 5).unwrap());
    }

}
