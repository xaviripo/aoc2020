use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines(file: &str) -> io::Result<impl Iterator<Item=String>> {
    let file = File::open(file)?;
    Ok(io::BufReader::new(file).lines().into_iter().flatten())
}
