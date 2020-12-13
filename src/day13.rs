use crate::lib;

pub const INPUT_FILE: &str = "input/13.txt";

/// Returns (earliest bus id you can take, waiting time)
fn times(earliest: &usize, bus_ids: &Vec<usize>) -> (usize, usize) {
    for time in *earliest.. {
        for &bus_id in &bus_ids[..] {
            if time % bus_id == 0 {
                return (bus_id, time - earliest);
            }
        }
    }
    // This should never happen
    (0,0)
}

fn parse_input(lines: Vec<String>) -> Option<(usize, Vec<usize>)> {
    Some((
        lines.get(0)?.parse::<usize>().ok()?,
        lines
        .get(1)?
        .split(',')
        .filter(|&s| s != "x")
        .try_fold(vec![], |mut acc, cur| {
            acc.push(cur.parse().ok()?);
            Some(acc)
        })?
    ))
}

pub fn run1(file: &str) -> Option<usize> {
    let lines: Vec<String> = lib::read_lines(file).ok()?.collect();
    let (earliest, bus_ids) = parse_input(lines).unwrap();
    let times = times(&earliest, &bus_ids);
    Some(times.0 * times.1)
}

fn parse_list(list: String) -> Vec<Option<usize>> {
    list
    .split(',')
    .map(|s| s.parse::<usize>().ok())
    .collect()
}

fn euclides(a: isize, b: isize) -> (isize, isize, isize) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x, y) = euclides(b % a, a);
        (gcd, y - (b / a) * x, x)
    }
}
 
fn inverse(x: isize, m: isize) -> Option<isize> {
    let (gcd, x, _) = euclides(x, m);
    if gcd == 1 {
        Some((x % m + m) % m)
    } else {
        None
    }
}
 
fn chinese_remainder(list: Vec<(isize, isize)>) -> Option<isize> {

    let prod: isize = list.iter().map(|(_, modulus)| modulus).product();

    let sum = list.into_iter().try_fold(0, |acc, (residue, modulus)| {
        let p = prod / modulus;
        Some(acc + residue * inverse(p, modulus)? * p)
    });

    Some(sum? % prod)

}

fn times2(input: Vec<Option<usize>>) -> Option<isize> {

    let list: Vec<(isize, isize)> = input
    .into_iter()
    .enumerate()
    .filter(|(_, option)| *option != None)
    .map(|(i, some)| {
        let value = some.unwrap() as isize;
        (value - i as isize, value)
    })
    .collect();

    chinese_remainder(list)

}

pub fn run2(file: &str) -> Option<isize> {
    let lines: Vec<String> = lib::read_lines(file).ok()?.collect();
    times2(parse_list(lines[1].clone()))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str =
"939
7,13,x,x,59,x,31,19";

    #[test]
    fn run1() {
        let lines: Vec<String> = INPUT_1.lines().map(|s| s.to_string()).collect();
        let (earliest, bus_ids) = parse_input(lines).unwrap();
        let times = times(&earliest, &bus_ids);
        assert_eq!(295, times.0 * times.1);
    }

    #[test]
    fn run2() {
        let cases = [
            ("7,13,x,x,59,x,31,19", 1068781),
            ("17,x,13,19", 3417),
            ("67,7,59,61", 754018),
            ("67,x,7,59,61", 779210),
            ("67,7,x,59,61", 1261476),
            ("1789,37,47,1889", 1202161486),
        ];
        for (input, output) in cases.iter() {
            assert_eq!(Some(*output as isize), times2(parse_list(input.to_string())));
        }
    }

}
