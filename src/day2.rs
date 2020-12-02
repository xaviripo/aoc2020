use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE: &str = "input/2.txt";

pub fn run1() -> Option<u32> {

    // Read the input file into a vector
    if let Ok(file) = File::open(INPUT_FILE) {
        Some(io::BufReader::new(file).lines().into_iter().filter(|maybe_line| {
            if let Ok(line) = maybe_line {

                // Rust: [is safe]
                // Me: let there be `.unwrap()`
                let parts1: Vec<&str> = line.split(": ").collect();
                let parts2: Vec<&str> = parts1[0].split(" ").collect();
                let parts3: Vec<u8> = parts2[0].split("-").map(|s| s.parse::<u8>().unwrap()).collect();

                let password = parts1[1];
                let letter = parts2[1].parse::<char>().unwrap();
                let min = parts3[0];
                let max = parts3[1];

                let amount = password.chars().filter(|&l| l == letter).count() as u8;

                amount >= min && amount <= max

            } else {
                // No damage in returning an extra false
                false
            }
        }).count() as u32)
    } else {
        None
    }

}

pub fn run2() -> Option<u32> {

    // Read the input file into a vector
    if let Ok(file) = File::open(INPUT_FILE) {
        Some(io::BufReader::new(file).lines().into_iter().filter(|maybe_line| {
            if let Ok(line) = maybe_line {

                // Rust: [is safe]
                // Me: let there be `.unwrap()`
                let parts1: Vec<&str> = line.split(": ").collect();
                let parts2: Vec<&str> = parts1[0].split(" ").collect();
                let parts3: Vec<usize> = parts2[0].split("-").map(|s| s.parse::<usize>().unwrap()).collect();

                let password: Vec<char> = parts1[1].chars().collect();
                let letter = parts2[1].parse::<char>().unwrap();
                let first_position = parts3[0];
                let second_position = parts3[1];

                let first_matches = password[first_position - 1] == letter;
                let second_matches = password[second_position - 1] == letter;

                (first_matches && !second_matches) || (!first_matches && second_matches)

            } else {
                // No damage in returning an extra false
                false
            }
        }).count() as u32)
    } else {
        None
    }

}
