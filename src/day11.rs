use std::iter;

use crate::lib;

pub const INPUT_FILE: &str = "input/11.txt";

const FLOOR: char = '.';
const FREE: char = 'L';
const OCCUPIED: char = '#';

fn next1(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut next_grid: Vec<Vec<char>> = grid.clone();
    for (i, row) in grid.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == FLOOR {
                continue;
            }
            let horizontal = if i == 0 { vec![i, i+1] } else if i == next_grid.len() - 1 { vec![i-1, i] } else { vec![i-1, i, i+1] };
            let vertical = if j == 0 { vec![j, j+1] } else if j == row.len() - 1 { vec![j-1, j] } else { vec![j-1, j, j+1] };
            let neighbors_with_center = horizontal.iter().flat_map(|i_| iter::repeat(i_).zip(vertical.iter()));
            let neighbors = neighbors_with_center.filter(|(&i_, &j_)| (i, j) != (i_, j_));
            next_grid[i][j] = match neighbors.filter(|(&i_, &j_)| grid[i_][j_] == OCCUPIED).count() {
                0 => OCCUPIED,
                4..=8 => FREE,
                _ => value,
            }
        }
    };

    next_grid
}

fn next2(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut next_grid: Vec<Vec<char>> = grid.clone();
    for (i, row) in grid.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == FLOOR {
                continue;
            }

            let top_i = (0..i).rev();
            let middle_i = iter::repeat(i);
            let bottom_i = (i..next_grid.len()).skip(1);

            let left_j = (0..j).rev();
            let middle_j = iter::repeat(j);
            let right_j = (j..row.len()).skip(1);

            // For each arm of the "windmill", we want to go from the center outside, and skip the center
            let tl = top_i.clone().zip(left_j.clone());
            let t = top_i.clone().zip(middle_j.clone());
            let tr = top_i.clone().zip(right_j.clone());
            let r = middle_i.clone().zip(right_j.clone());
            let br = bottom_i.clone().zip(right_j.clone());
            let b = bottom_i.clone().zip(middle_j.clone());
            let bl = bottom_i.clone().zip(left_j.clone());
            let l = middle_i.clone().zip(left_j.clone());

            let windmill: Vec<Vec<(usize, usize)>> = vec![
                tl.collect(),
                t.collect(),
                tr.collect(),
                r.collect(),
                br.collect(),
                b.collect(),
                bl.collect(),
                l.collect()
            ];

            let total_occupied = windmill.into_iter().filter(|line| {
                match line.iter()
                .map(|(i_, j_)| grid[*i_][*j_])
                .filter(|value| *value != FLOOR)
                .next() {
                    Some(OCCUPIED) => true,
                    _ => false,
                }
            }).count();

            next_grid[i][j] = match total_occupied {
                0 => OCCUPIED,
                5..=8 => FREE,
                _ => value,
            }
        }
    };

    next_grid
}

fn next(grid: &Vec<Vec<char>>, first: bool) -> Vec<Vec<char>> {
    if first {
        next1(grid)
    } else {
        next2(grid)
    }
}

fn stabilize(grid: Vec<Vec<char>>, first: bool) -> Vec<Vec<char>> {
    let mut current_grid = grid;
    let mut next_grid = next(&current_grid, first);
    while next_grid != current_grid {
        current_grid = next_grid;
        next_grid = next(&current_grid, first);
    }
    next_grid
}

fn count_occupied(grid: &Vec<Vec<char>>) -> usize {
    grid.iter().flatten().filter(|&&value| value == OCCUPIED).count()
}

pub fn run1(file: &str) -> std::io::Result<usize> {
    let grid: Vec<Vec<char>> = lib::read_lines(file)?.map(|s| s.chars().collect::<Vec<char>>()).collect();
    Ok(count_occupied(&stabilize(grid, true)))
}

pub fn run2(file: &str) -> std::io::Result<usize> {
    let grid: Vec<Vec<char>> = lib::read_lines(file)?.map(|s| s.chars().collect::<Vec<char>>()).collect();
    Ok(count_occupied(&stabilize(grid, false)))
}

#[cfg(test)]
mod test {
    use super::{stabilize, count_occupied};


    const INPUT_1: &str =
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn run1() {
        let grid: Vec<Vec<char>> = INPUT_1.lines().map(|s| s.chars().collect::<Vec<char>>()).collect();
        assert_eq!(37, count_occupied(&stabilize(grid, true)));
    }

    #[test]
    fn run2() {
        let grid: Vec<Vec<char>> = INPUT_1.lines().map(|s| s.chars().collect::<Vec<char>>()).collect();
        assert_eq!(26, count_occupied(&stabilize(grid, false)));
    }

}
