mod day1;
mod day2;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match (args[1].as_str(), args[2].as_str()) {
        ("1", "1") => { println!("{}", day1::run1().unwrap()); }
        ("1", "2") => { println!("{}", day1::run2().unwrap()); }
        ("2", "1") => { println!("{}", day2::run1().unwrap()); }
        ("2", "2") => { println!("{}", day2::run2().unwrap()); }
        _ => { panic!("Unknown problem {:?}", &args[1]) }
    }
}
