use std::iter;

use aoc2025::grid::{Grid, Idx};

fn part_1(input: &str) -> u64 {
    let grid = Grid::from_rows(input.lines().map(|line| line.split_whitespace()));
    let size = grid.size();

    (0..size.x)
        .map(|x| {
            let numbers = (0..size.y - 1).map(|y| grid[Idx { x, y }].parse::<u64>().unwrap());
            let operator = grid[Idx { x, y: size.y - 1 }];
            match operator {
                "*" => numbers.product::<u64>(),
                "+" => numbers.sum::<u64>(),
                _ => panic!("{operator:?}"),
            }
        })
        .sum()
}

fn part_2(input: &str) -> u64 {
    let grid = Grid::from_rows(input.lines().map(|line| line.chars()));
    let size = grid.size();

    let blank_xs = (0..size.x)
        .filter(|&x| (0..size.y).all(|y| grid[Idx { x, y }] == ' '))
        .collect::<Vec<_>>();

    (iter::once(0).chain(blank_xs.iter().copied().map(|x| x + 1)))
        .zip(blank_xs.iter().copied().chain(iter::once(size.x)))
        .map(|(from_x, to_x)| {
            let numbers = (from_x..to_x).map(|x| {
                (0..size.y - 1)
                    .map(|y| grid[Idx { x, y }])
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
            });
            let operator = grid[Idx {
                x: from_x,
                y: size.y - 1,
            }];
            match operator {
                '*' => numbers.product::<u64>(),
                '+' => numbers.sum::<u64>(),
                _ => panic!("{operator:?}"),
            }
        })
        .sum()
}

fn run(input: &str) -> (u64, u64) {
    (part_1(input), part_2(input))
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(&aoc::example!(0)), 4277556);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(&aoc::example!(0)), 3263827);
}

aoc::main!(run);
