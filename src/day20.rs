use std::collections::HashMap;
use std::convert::TryInto;

use crate::lib;

pub const INPUT_FILE: &str = "input/20.txt";

// "Tiles at the edge of the image also have this border, but the outermost edges won't line up with any other tiles."
// Let's pray this is true and look for all the tiles with exactly 2 unalignable borders, then multiply their ids.

type Border = Vec<bool>;

#[derive(Debug, PartialEq, Clone)]
struct Tile(Border, Border, Border, Border);

fn fits(this: &Border, other: &Border) -> bool {
    let mut this_rev = this.clone();
    this_rev.reverse();
    this == other || &this_rev == other
}

fn border_tiles(tiles: HashMap<usize, Tile>) -> Vec<usize> {
    tiles
    .clone()
    .into_iter()
    .map(|(id, tile)| {

        let others_borders: Vec<_> = tiles
        .clone()
        .into_iter()
        .filter(|(other_id, _)| *other_id != id)
        .map(|(_, other)| vec![other.0, other.1, other.2, other.3])
        .flatten()
        .collect();

        let tile_clone = tile.clone();
        let fitting_sides = vec![tile_clone.0, tile_clone.1, tile_clone.2, tile_clone.3]
        .into_iter()
        .filter(|this| {
            others_borders
            .clone()
            .into_iter()
            .find(|other| {
                fits(this, other)
            }) != None
        })
        .count();

        (id, tile, fitting_sides)

    })
    .filter(|(_, _, fitting_sides)| *fitting_sides == 2)
    .map(|(id, _, _)| id)
    .collect()
}

fn parse_tile(lines: Vec<String>) -> (usize, Tile) {
    let id: usize = lines[0][5..9].to_string().parse().unwrap();
    let tile = Tile(
        lines[1].chars().map(|c| match c {
            '.' => false,
            _ => true,
        })
        .collect::<Vec<_>>().try_into().unwrap(),
        lines[10].chars().map(|c| match c {
            '.' => false,
            _ => true,
        })
        .collect::<Vec<_>>().try_into().unwrap(),
        lines[1..11].into_iter().map(|line| match line.chars().nth(0).unwrap() {
            '.' => false,
            _ => true,
        })
        .collect::<Vec<_>>().try_into().unwrap(),
        lines[1..11].into_iter().map(|line| match line.chars().nth(9).unwrap() {
            '.' => false,
            _ => true,
        })
        .collect::<Vec<_>>().try_into().unwrap(),
    );
    (id, tile)
}

fn parse<T: Iterator<Item=String>>(lines: T) -> HashMap<usize, Tile> {
    lines
    .collect::<Vec<_>>()
    .chunks(12)
    .map(|chunk| chunk.into_iter().map(|s| s.clone()).collect())
    .map(parse_tile)
    .collect()
}

pub fn run1(file: &str) -> std::io::Result<usize> {
    let input = lib::read_lines(file)?;
    Ok(border_tiles(parse(input)).into_iter().product())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str =
"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
";

    #[test]
    fn test_fits() {
        let input = INPUT_1.lines().map(|s| s.to_string());
        let tiles = parse(input);
        let mut reversed = tiles.get(&3079).unwrap().2.clone();
        reversed.reverse();
        assert!(fits(
            &tiles.get(&3079).unwrap().2,
            &tiles.get(&2311).unwrap().3,
        ));
        assert!(fits(
            &reversed,
            &tiles.get(&2311).unwrap().3,
        ));
    }

    #[test]
    fn run1() {
        let input = INPUT_1.lines().map(|s| s.to_string());
        assert_eq!(20899048083289usize, border_tiles(parse(input)).into_iter().product());
    }

}


// 3079 .1
// 2311 .3