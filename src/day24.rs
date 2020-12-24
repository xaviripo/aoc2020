use std::collections::HashSet;

use crate::lib;

pub const INPUT_FILE: &str = "input/24.txt";

/// We use three axis hexagonal coordinates
/// With this system, when moving in any of the three axis of neighbor squares, not one but two coordinates change; the third is invariant.
/// See: https://catlikecoding.com/unity/tutorials/hex-map/part-1/hexagonal-coordinates/cube-diagram.png
/// m: invariant in the main diagonal
/// s: invariant in the secondary diagonal
/// h: invariant in the horizontal
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Coord(isize, isize, isize);

enum Direction {
    W,
    E,
    NW,
    NE,
    SW,
    SE,
}

fn walk(Coord(m, s, h): Coord, direction: Direction) -> Coord {
    use Direction::*;
    match direction {
        W => Coord(m - 1, s + 1, h),
        E => Coord(m + 1, s - 1, h),
        NW => Coord(m - 1, s, h + 1),
        NE => Coord(m, s - 1, h + 1),
        SW => Coord(m, s + 1, h - 1),
        SE => Coord(m + 1, s, h - 1),
    }
}

fn coordinate(directions: Vec<Direction>) -> Coord {
    directions.into_iter()
    .fold(Coord(0, 0, 0), walk)
}

fn parse(line: String) -> Vec<Direction> {
    let mut chars = line.chars();
    let mut directions = vec![];
    while let Some(c) = chars.next() {
        directions.push(match c {
            'n' => match chars.next() {
                Some('e') => Direction::NE,
                Some('w') => Direction::NW,
                _ => panic!(),
            },
            's' => match chars.next() {
                Some('e') => Direction::SE,
                Some('w') => Direction::SW,
                _ => panic!(),
            },
            'e' => Direction::E,
            'w' => Direction::W,
            _ => panic!(),
        });
    }
    directions
}

fn black<T: Iterator<Item=String>>(lines: T) -> HashSet<Coord> {
    lines
    .map(parse)
    .map(coordinate)
    .fold(HashSet::new(), |mut black, coord| {
        if !black.remove(&coord) {
            black.insert(coord);
        }
        black
    })
}

fn neighbors(coord: Coord) -> Vec<Coord> {
    use Direction::*;
    vec![
        W,
        E,
        NW,
        NE,
        SW,
        SE,
    ].into_iter()
    .map(|direction| walk(coord, direction))
    .collect()
}

fn flip(black: HashSet<Coord>) -> HashSet<Coord> {

    let candidates: Vec<Coord> = black.clone().into_iter()
    .map(|coord| neighbors(coord))
    .flatten()
    .chain(black.clone().into_iter())
    .collect();

    candidates.into_iter()
    .fold(HashSet::new(), |mut new_black, coord| {
        let black_neighbors = neighbors(coord).into_iter()
        .filter(|neighbor| black.contains(neighbor))
        .count();

        if match black_neighbors {
            1 => black.contains(&coord),
            2 => true,
            _ => false,
        } {
            new_black.insert(coord);
        }
        new_black
    })

}

fn flip_n(mut black: HashSet<Coord>, times: usize) -> HashSet<Coord> {
    for _ in 0..times {
        black = flip(black);
    }
    black
}

pub fn run1(file: &str) -> std::io::Result<usize> {
    let input = lib::read_lines(file)?;
    Ok(black(input).len())
}

pub fn run2(file: &str) -> std::io::Result<usize> {
    let input = lib::read_lines(file)?;
    Ok(flip_n(black(input), 100).len())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str =
"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
";

    #[test]
    fn run1() {
        let input = INPUT_1.lines().map(|s| s.to_string());
        assert_eq!(10, black(input).len());
    }

    #[test]
    fn run2() {
        let input = INPUT_1.lines().map(|s| s.to_string());
        assert_eq!(2208, flip_n(black(input), 100).len());
    }

}