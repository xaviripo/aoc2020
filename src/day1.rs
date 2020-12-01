use std::fs::File;
use std::io::{self, BufRead};

const SUM: u32 = 2020;
const INPUT_FILE: &str = "input/1.txt";

pub fn run1() -> Option<u32> {

    // Read the input file into a vector
    let mut numbers: Vec<u32> = vec![];
    if let Ok(file) = File::open(INPUT_FILE) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                if let Ok(number) = line.parse::<u32>() {
                    numbers.push(number);
                }
            }
        }
    }

    // Sort it
    numbers.sort_unstable();

    // Now look for the combinations:
    let mut i = 1;
    for first_number in &numbers {
        for second_number in &numbers[i..] {
            if first_number + second_number == SUM {
                return Some(first_number * second_number);
            }
        }
        i += 1;
    }

    None

}

pub fn run2() -> Option<u32> {

    // Read the input file into a vector
    let mut numbers: Vec<u32> = vec![];
    if let Ok(file) = File::open(INPUT_FILE) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                if let Ok(number) = line.parse::<u32>() {
                    numbers.push(number);
                }
            }
        }
    }

    // Sort it
    numbers.sort_unstable();

    // Now look for the combinations:
    let mut i = 1;
    for first_number in &numbers {
        let mut j = 2;
        for second_number in &numbers[i..] {
            for third_number in &numbers[j..] {
                if first_number + second_number + third_number == SUM {
                    return Some(first_number * second_number * third_number);
                }
            }
            j += 1;
        }
        i += 1;
    }

    None

}
