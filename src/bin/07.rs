use aoc2025::grid::{Grid, Idx};

fn run(input: &str) -> (u64, u64) {
    let mut grid = Grid::from_rows(input.lines().map(|line| line.chars()));
    let size = grid.size();

    let start_idx = (0..size.x)
        .map(|x| Idx::new(x, 0))
        .find(|&idx| grid[idx] == 'S')
        .unwrap();

    let mut part_1 = 0;
    grid[start_idx] = '|';
    for y in 0..size.y - 1 {
        for x in 0..size.x {
            let idx = Idx::new(x, y);
            if grid[idx] == '|' {
                if grid[idx.offset(0, 1)] == '^' {
                    part_1 += 1;
                    for dx in [-1, 1] {
                        if grid[idx.offset(dx, 1)] != '|' {
                            grid[idx.offset(dx, 1)] = '|';
                        }
                    }
                } else {
                    grid[idx.offset(0, 1)] = '|';
                }
            }
        }
    }

    let mut num_timelines = Grid::from_size_fn(size, |_| 1);
    for y in (0..size.y - 1).rev() {
        for x in 0..size.x {
            let idx = Idx::new(x, y);
            if grid[idx.offset(0, 1)] == '^' {
                num_timelines[idx] =
                    num_timelines[idx.offset(-1, 1)] + num_timelines[idx.offset(1, 1)];
            } else {
                num_timelines[idx] = num_timelines[idx.offset(0, 1)];
            }
        }
    }
    let part_2 = num_timelines[start_idx];

    (part_1, part_2)
}

#[test]
fn test_part_1() {
    assert_eq!(run(&aoc::example!(0)).0, 21);
}

#[test]
fn test_part_2() {
    assert_eq!(run(&aoc::example!(0)).1, 40);
}

aoc::main!(run);
