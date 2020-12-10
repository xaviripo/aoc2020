use std::iter;

use crate::lib;

pub const INPUT_FILE: &str = "input/10.txt";

fn differences(lines: Vec<String>) -> Vec<u64> {
    let mut numbers = lines
    .into_iter()
    .map(|l| l.parse::<u64>().unwrap())
    .collect::<Vec<u64>>();

    numbers.sort_unstable();
    let &max = numbers.iter().max().unwrap();

    let curs = iter::once(0).chain(numbers.clone().into_iter());
    let nexts = numbers.into_iter().chain(iter::once(max + 3));

    curs.zip(nexts).map(|(cur, next)| next - cur).collect()
}

fn process1(lines: Vec<String>) -> Option<usize> {

    let differences = differences(lines);

    let _1s = differences.clone().into_iter().filter(|&d| d == 1).count();
    let _3s = differences.into_iter().filter(|&d| d == 3).count();

    Some(_1s * _3s)

}

// For problem 2, we can group the adapters in blocks separated by 3 (because those CANNOT be avoided), see how many
// different paths there are within each of those blocks, and multiply all those numbers together
// Within each block, separations can be either 1, or 2, so their diffs will look like:
// e.g. [1, 2, 2, 2, 1, 1, 2, 1]
// We can further break these down when we encounter two consecutive 2s:
// e.g. [1, 2, 2, 2, 1, 1, 2, 1] -> [1, 2] (2) [2, 1, 1, 2, 1]
//                                          ^ the second 2 can be dropped
// This can be done because there is no combination that doesn't pick the adapter between the two 2s.
// Finally, we must count all the ways in which the elements of each array can be contiguously summed into <=3,
// e.g. [1, 2] -> { [1, 2], [3] }
// e.g. [2, 1, 1, 2, 1] -> { [2, 1, 1, 2, 1],
//                           [2, 1, 1, 3],
//                           [2, 1, 3, 1],
//                           [2, 2, 2, 1],
//                           [2, 2, 3],
//                           [3, 1, 2, 1],
//                           [3, 1, 3],
//                           [3, 3, 1] }
// This can be done recursively.

fn combinations(chunk: Vec<u64>) -> u64 {
    if chunk.len() <= 1 {
        return 1;
    }
    let mut sum: u64 = 0;
    sum += combinations(chunk.clone().into_iter().skip(1).collect());
    if chunk[0] + chunk[1] <= 3 {
        if let Some(first) = chunk.get(0) {
            if let Some(second) = chunk.get(1) {
                if first + second <= 3 {
                    let mut smaller_chunk = vec![first + second];
                    smaller_chunk.extend(chunk.into_iter().skip(2));
                    sum += combinations(smaller_chunk);
                }
            }
        }
    }
    sum
}

fn process2(lines: Vec<String>) -> u64 {

    let differences = differences(lines);

    let string = differences
    .into_iter()

    // easiest way to split by 3s? convert into a string, apparently
    .map(|n| n.to_string().chars().nth(0).unwrap())
    .collect::<String>();

    let chunks = string.split('3');

    chunks
    .map(|chunk| {
        chunk
        .replace("22", "2X")
        .split('X')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    })
    .flatten()
    .map(|s|
        s
        .chars()
        .map(|c|
            c
            .to_string()
            .parse::<u64>()
            .unwrap()
        )
        .collect::<Vec<u64>>()
    )
    .map(|chunk| combinations(chunk))
    .product::<u64>()

}

pub fn run1(file: &str) -> Option<usize> {
    process1(lib::read_lines(file).ok()?.collect())
}

pub fn run2(file: &str) -> std::io::Result<u64> {
    Ok(process2(lib::read_lines(file)?.collect()))
}

#[cfg(test)]
mod test {
    use super::{process1, process2};


    const INPUT_1: &str =
"16
10
15
5
1
11
7
19
6
12
4";

    const INPUT_2: &str =
"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn run1_1() {
        assert_eq!(Some(35), process1(INPUT_1.lines().map(|s| s.to_string()).collect()));
    }

    #[test]
    fn run1_2() {
        assert_eq!(Some(220), process1(INPUT_2.lines().map(|s| s.to_string()).collect()));
    }

    #[test]
    fn run2_1() {
        assert_eq!(8, process2(INPUT_1.lines().map(|s| s.to_string()).collect()));
    }

    #[test]
    fn run2_2() {
        assert_eq!(19208, process2(INPUT_2.lines().map(|s| s.to_string()).collect()));
    }

}
