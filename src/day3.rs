use crate::lib;

pub const INPUT_FILE: &str = "input/3.txt";

fn count_trees(lines: impl Iterator<Item=String>, right: usize, down: usize) -> usize {
    let mut position: usize = 0;
    lines
        .step_by(down)
        .filter(|line| {
            let is_tree = line.chars().nth(position % line.len()).unwrap() == '#';
            position += right;
            is_tree
        })
        .count()
}

fn count_trees_multislope(lines: impl Iterator<Item=String>) -> usize {
    let lines: Vec<String> = lines.collect();
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(right, down)| count_trees(lines.clone().into_iter(), right, down))
        .product()
}

pub fn run1(file: &str) -> std::io::Result<usize> {
    Ok(count_trees(lib::read_lines(file)?, 3, 1))
}

pub fn run2(file: &str) -> std::io::Result<usize> {
    Ok(count_trees_multislope(lib::read_lines(file)?))
}

#[cfg(test)]
mod test {

    use super::{count_trees, count_trees_multislope};

    #[test]
    fn run1() {

        let lines = "..##.......
                          #...#...#..
                          .#....#..#.
                          ..#.#...#.#
                          .#...##..#.
                          ..#.##.....
                          .#.#.#....#
                          .#........#
                          #.##...#...
                          #...##....#
                          .#..#...#.#";

        assert_eq!(count_trees(lines.split('\n').map(|s| s.trim().to_string()), 3, 1), 7);
    }

    #[test]
    fn run2() {

        let lines = "..##.......
                          #...#...#..
                          .#....#..#.
                          ..#.#...#.#
                          .#...##..#.
                          ..#.##.....
                          .#.#.#....#
                          .#........#
                          #.##...#...
                          #...##....#
                          .#..#...#.#";

        assert_eq!(count_trees_multislope(lines.split('\n').map(|s| s.trim().to_string())), 336);
    }

}
