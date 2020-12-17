use std::{collections::{HashMap, HashSet}, hash::Hash};

use crate::lib;

pub const INPUT_FILE: &str = "input/17.txt";

trait NCube {
    fn neighborhood(&self) -> Vec<Self>
    where Self: Sized;
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Cube(isize, isize, isize);

impl NCube for Cube {
    /// Returns the 3*3*3 slice centered around self
    fn neighborhood(&self) -> Vec<Cube> {
        let &Cube(x, y, z) = self;
        let xs = [x-1, x, x+1];
        let ys = [y-1, y, y+1];
        let zs = [z-1, z, z+1];

        let mut neighbors = vec![];
        for &x in xs.iter() {
            for &y in ys.iter() {
                for &z in zs.iter() {
                    neighbors.push(Cube(x, y, z));
                }
            }
        }

        neighbors

    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct HyperCube(isize, isize, isize, isize);

impl NCube for HyperCube {
    /// Returns the 3*3*3 slice centered around self
    fn neighborhood(&self) -> Vec<HyperCube> {
        let &HyperCube(x, y, z, w) = self;
        let xs = [x-1, x, x+1];
        let ys = [y-1, y, y+1];
        let zs = [z-1, z, z+1];
        let ws = [w-1, w, w+1];

        let mut neighbors = vec![];
        for &x in xs.iter() {
            for &y in ys.iter() {
                for &z in zs.iter() {
                    for &w in ws.iter() {
                        neighbors.push(HyperCube(x, y, z, w));
                    }
                }
            }
        }

        neighbors

    }
}

#[derive(Clone)]
struct Grid<T>(HashSet<T>);

struct Sequence<T>(Grid<T>);

impl<T: NCube + Hash + Eq + Clone> Iterator for Sequence<T> {
    type Item = Grid<T>;

    fn next(&mut self) -> Option<Grid<T>> {
        let set = self.0.0
        .iter()
        .map(|cube| {
            cube.neighborhood()
        })
        .flatten()
        .fold(HashMap::new(), |mut count, cube| {
            let &amount = count.get(&cube).unwrap_or(&0usize);
            count.insert(cube, amount + 1);
            count
        })
        .into_iter()
        .fold(HashSet::new(), |mut grid, (cube, neighbors)| {
            if self.0.0.contains(&cube) {
                // Remember that neighbors counts the center cube itself!
                if neighbors == 3 || neighbors == 4 {
                    grid.insert(cube);
                }
            } else {
                if neighbors == 3 {
                    grid.insert(cube);
                }
            }
            grid
        });

        let grid = Grid(set);
        self.0 = grid.clone();
        Some(grid)

    }
}

fn run<T: NCube + Hash + Eq + Clone>(grid: Grid<T>, iterations: usize) -> Grid<T> {
    Sequence(grid).nth(iterations - 1).unwrap()
}

fn text_to_grid_3(text: Vec<String>) -> Grid<Cube> {

    let mut grid: HashSet<Cube> = HashSet::new();

    for (y, line) in text.into_iter().enumerate() {
        for (x, content) in line.chars().enumerate() {
            if content == '#' {
                grid.insert(Cube(x as isize, y as isize, 0));
            }
        }
    }

    Grid(grid)

}

fn text_to_grid_4(text: Vec<String>) -> Grid<HyperCube> {

    let mut grid: HashSet<HyperCube> = HashSet::new();

    for (y, line) in text.into_iter().enumerate() {
        for (x, content) in line.chars().enumerate() {
            if content == '#' {
                grid.insert(HyperCube(x as isize, y as isize, 0, 0));
            }
        }
    }

    Grid(grid)

}

pub fn run1(file: &str) -> std::io::Result<usize> {
    let grid = text_to_grid_3(lib::read_lines(file)?.collect());
    Ok(run(grid, 6).0.len())
}

pub fn run2(file: &str) -> std::io::Result<usize> {
    let grid = text_to_grid_4(lib::read_lines(file)?.collect());
    Ok(run(grid, 6).0.len())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str =
".#.
..#
###";

    #[test]
    fn run1() {
        let grid = text_to_grid_3(INPUT_1.lines().map(|s| s.to_string()).collect());
        assert_eq!(112, run(grid, 6).0.len());
    }

    #[test]
    fn run2() {
        let grid = text_to_grid_4(INPUT_1.lines().map(|s| s.to_string()).collect());
        assert_eq!(848, run(grid, 6).0.len());
    }

}
