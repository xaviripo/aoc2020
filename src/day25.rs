use crate::lib;

pub const INPUT_FILE: &str = "input/25.txt";

fn step(value: usize, subject_number: usize) -> usize {
    (value * subject_number) % 20201227
}

fn transform(subject_number: usize, loop_size: usize) -> usize {
    (0..loop_size)
    .fold(1, |value, _| step(value, subject_number))
}

fn loop_size(public_key: usize) -> usize {
    let mut value = 1;
    let mut loop_size = 0;
    while value != public_key {
        value = step(value, 7);
        loop_size += 1;
    }
    loop_size
}

fn encryption_key(a_public_key: usize, b_public_key: usize) -> usize {

    let a_loop_size = loop_size(a_public_key);
    transform(b_public_key, a_loop_size)

}

pub fn run1(file: &str) -> std::io::Result<usize> {
    let input: Vec<usize> = lib::read_lines(file)?.map(|s| s.parse().unwrap()).collect();
    Ok(encryption_key(input[0], input[1]))
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str =
"5764801
17807724";

    #[test]
    fn run1() {

        let input: Vec<usize> = INPUT_1.lines().map(|s| s.parse().unwrap()).collect();

        let card_public_key = input[0];
        let door_public_key = input[1];

        let card_encryption_key = encryption_key(card_public_key, door_public_key);
        let door_encryption_key = encryption_key(door_public_key, card_public_key);

        assert_eq!(card_encryption_key, door_encryption_key);
        assert_eq!(14897079, card_encryption_key)

    }

}
