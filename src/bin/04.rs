use std::fmt;

use aoc2025::grid::Grid;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Cell {
    Free,
    Roll,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Free,
            '@' => Cell::Roll,
            _ => panic!("{c}"),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Free => '.',
                Self::Roll => '@',
            }
        )
    }
}

fn run(input: &str) -> (usize, usize) {
    let grid = Grid::from_rows(input.lines().map(|line| line.chars().map(Cell::from)));
    let part_1 = grid
        .enumerate()
        .filter(|(idx, cell)| {
            **cell == Cell::Roll
                && idx
                    .eight_neighbors()
                    .filter(|&neigh_idx| grid.try_get(neigh_idx) == Some(&Cell::Roll))
                    .count()
                    < 4
        })
        .count();

    let mut part_2 = 0;
    let mut grid = grid;
    let mut num_adjacent_rolls = Grid::from_size_fn(grid.size(), |idx| {
        idx.eight_neighbors()
            .filter(|&neigh_idx| grid.try_get(neigh_idx) == Some(&Cell::Roll))
            .count()
    });
    let mut stack = grid
        .enumerate()
        .filter_map(|(idx, &cell)| {
            if cell == Cell::Roll && num_adjacent_rolls[idx] < 4 {
                Some(idx)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    while let Some(idx) = stack.pop() {
        debug_assert_eq!(grid[idx], Cell::Roll);
        grid[idx] = Cell::Free;
        part_2 += 1;
        for neigh_idx in idx.eight_neighbors() {
            if grid.try_get(neigh_idx) == Some(&Cell::Roll) {
                debug_assert!(num_adjacent_rolls[neigh_idx] > 0);
                num_adjacent_rolls[neigh_idx] -= 1;
                if num_adjacent_rolls[neigh_idx] == 3 {
                    stack.push(neigh_idx);
                }
            }
        }
    }

    (part_1, part_2)
}

#[test]
fn test_part_1() {
    assert_eq!(run(&aoc::example!(0)).0, 13);
}

#[test]
fn test_part_2() {
    assert_eq!(run(&aoc::example!(0)).1, 43);
}

aoc::main!(run);
