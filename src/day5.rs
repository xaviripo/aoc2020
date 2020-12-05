use crate::lib;

pub const INPUT_FILE: &str = "input/5.txt";

fn seat_id(seat: &str) -> Option<u64> {
    let row_string = &seat[..7]
    .replace("F", "0")
    .replace("B", "1");
    let row = u64::from_str_radix(row_string, 2).ok()?;

    let column_string = &seat[7..]
    .replace("L", "0")
    .replace("R", "1");
    let column = u64::from_str_radix(column_string, 2).ok()?;

    Some(row * 8 + column)
}

pub fn run1(file: &str) -> Option<u64> {
    lib::read_lines(file).ok()?
    .map(|line| seat_id(line.as_str()))
    .max()?
}

pub fn run2(file: &str) -> Option<u64> {
    let seats: Vec<u64> = lib::read_lines(file).ok()?
    .map(|line| seat_id(line.as_str()))
    .collect::<Option<Vec<u64>>>()?;
    (8..127*8).find(|&id|
        !seats.contains(&id) && // Not in the list
        seats.contains(&(id - 1)) && seats.contains(&(id + 1)) // Both previous and next ids are present
    )
}

#[cfg(test)]
mod test {
    use super::seat_id;


    const SEATS: [&str; 3] = [
        "BFFFBBFRRR",
        "FFFBBBFRRR",
        "BBFFBBFRLL",
    ];

    const IDS: [u64; 3] = [
        567,
        119,
        820,
    ];

    #[test]
    fn run1() {
        let computed_ids: Vec<u64> = SEATS.iter().map(|&seat| seat_id(seat).unwrap_or(0)).collect();
        assert_eq!(computed_ids, IDS.iter().cloned().collect::<Vec<u64>>());
    }

}
