#[macro_use] extern crate lazy_static;

mod lib;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match (args[1].as_str(), args[2].as_str()) {
        ("1", "1") => { println!("{}", day1::run1().unwrap()); }
        ("1", "2") => { println!("{}", day1::run2().unwrap()); }
        ("2", "1") => { println!("{}", day2::run1().unwrap()); }
        ("2", "2") => { println!("{}", day2::run2().unwrap()); }
        ("3", "1") => { println!("{}", day3::run1(day3::INPUT_FILE).unwrap()); }
        ("3", "2") => { println!("{}", day3::run2(day3::INPUT_FILE).unwrap()); }
        ("4", "1") => { println!("{}", day4::run1(day4::INPUT_FILE).unwrap()); }
        ("4", "2") => { println!("{}", day4::run2(day4::INPUT_FILE).unwrap()); }
        ("5", "1") => { println!("{}", day5::run1(day5::INPUT_FILE).unwrap()); }
        ("5", "2") => { println!("{}", day5::run2(day5::INPUT_FILE).unwrap()); }
        ("6", "1") => { println!("{}", day6::run1(day6::INPUT_FILE).unwrap()); }
        ("6", "2") => { println!("{}", day6::run2(day6::INPUT_FILE).unwrap()); }
        ("7", "1") => { println!("{}", day7::run1(day7::INPUT_FILE).unwrap()); }
        ("7", "2") => { println!("{}", day7::run2(day7::INPUT_FILE).unwrap()); }
        ("8", "1") => { println!("{}", day8::run1(day8::INPUT_FILE).unwrap()); }
        ("8", "2") => { println!("{}", day8::run2(day8::INPUT_FILE).unwrap()); }
        ("9", "1") => { println!("{}", day9::run1(day9::INPUT_FILE).unwrap()); }
        ("9", "2") => { println!("{}", day9::run2(day9::INPUT_FILE).unwrap()); }
        ("10", "1") => { println!("{}", day10::run1(day10::INPUT_FILE).unwrap()); }
        ("10", "2") => { println!("{}", day10::run2(day10::INPUT_FILE).unwrap()); }
        ("11", "1") => { println!("{}", day11::run1(day11::INPUT_FILE).unwrap()); }
        ("11", "2") => { println!("{}", day11::run2(day11::INPUT_FILE).unwrap()); }
        ("12", "1") => { println!("{}", day12::part1::run(day12::INPUT_FILE).unwrap()); }
        ("12", "2") => { println!("{}", day12::part2::run(day12::INPUT_FILE).unwrap()); }
        ("13", "1") => { println!("{}", day13::run1(day13::INPUT_FILE).unwrap()); }
        ("13", "2") => { println!("{}", day13::run2(day13::INPUT_FILE).unwrap()); }
        ("14", "1") => { println!("{}", day14::run1(day14::INPUT_FILE).unwrap()); }
        ("14", "2") => { println!("{}", day14::run2(day14::INPUT_FILE).unwrap()); }
        ("15", "1") => { println!("{}", day15::run1(day15::INPUT_FILE).unwrap()); }
        ("15", "2") => { println!("{}", day15::run2(day15::INPUT_FILE).unwrap()); }
        ("16", "1") => { println!("{}", day16::run1(day16::INPUT_FILE).unwrap()); }
        ("16", "2") => { println!("{}", day16::run2(day16::INPUT_FILE).unwrap()); }
        ("17", "1") => { println!("{}", day17::run1(day17::INPUT_FILE).unwrap()); }
        ("17", "2") => { println!("{}", day17::run2(day17::INPUT_FILE).unwrap()); }
        ("18", "1") => { println!("{}", day18::run1(day18::INPUT_FILE).unwrap()); }
        ("18", "2") => { println!("{}", day18::run2(day18::INPUT_FILE).unwrap()); }
        ("19", "1") => { println!("{}", day19::run1(day19::INPUT_FILE).unwrap()); }
        ("19", "2") => { println!("{}", day19::run2(day19::INPUT_FILE).unwrap()); }
        ("20", "1") => { println!("{}", day20::run1(day20::INPUT_FILE).unwrap()); }
        // ("20", "2") => { println!("{}", day20::run2(day20::INPUT_FILE).unwrap()); }
        ("21", "1") => { println!("{}", day21::run1(day21::INPUT_FILE).unwrap()); }
        ("21", "2") => { println!("{}", day21::run2(day21::INPUT_FILE).unwrap()); }
        ("22", "1") => { println!("{}", day22::part1::run(day22::INPUT_FILE).unwrap()); }
        ("22", "2") => { println!("{}", day22::part2::run(day22::INPUT_FILE).unwrap()); }
        ("23", "1") => { println!("{}", day23::run1(day23::INPUT_FILE).unwrap()); }
        ("23", "2") => { println!("{}", day23::run2(day23::INPUT_FILE).unwrap()); }
        ("24", "1") => { println!("{}", day24::run1(day24::INPUT_FILE).unwrap()); }
        ("24", "2") => { println!("{}", day24::run2(day24::INPUT_FILE).unwrap()); }
        ("25", "1") => { println!("{}", day25::run1(day25::INPUT_FILE).unwrap()); }
        _ => { panic!("Unknown problem {:?} {:?}", &args[1], &args[2]) }
    }
}
