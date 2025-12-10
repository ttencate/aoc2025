use std::{
    fmt,
    ops::{Index, IndexMut},
};

#[derive(Copy, Clone, Debug)]
struct Idx {
    x: isize,
    y: isize,
}

impl Idx {
    fn offset(self, dx: isize, dy: isize) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn eight_neighbors(&self) -> impl Iterator<Item = Self> {
        [
            self.offset(-1, -1),
            self.offset(0, -1),
            self.offset(1, -1),
            self.offset(-1, 0),
            self.offset(1, 0),
            self.offset(-1, 1),
            self.offset(0, 1),
            self.offset(1, 1),
        ]
        .into_iter()
    }

    fn indices_below(&self) -> impl Iterator<Item = Self> {
        (0..self.y).flat_map(move |y| (0..self.x).map(move |x| Idx { x, y }))
    }
}

struct Grid<T> {
    size: Idx,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    fn from_rows(rows: impl Iterator<Item = impl Iterator<Item = T>>) -> Self {
        let mut cells = Vec::new();
        let mut ny = 0;
        let mut nx = 0;
        for row in rows {
            for cell in row {
                cells.push(cell);
                if ny == 0 {
                    nx += 1;
                }
            }
            ny += 1;
            assert_eq!(nx * ny, cells.len());
        }
        Self {
            size: Idx {
                x: nx.try_into().unwrap(),
                y: ny.try_into().unwrap(),
            },
            cells,
        }
    }

    fn from_size_fn(size: Idx, f: impl FnMut(Idx) -> T) -> Self {
        Self {
            size,
            cells: size.indices_below().map(f).collect(),
        }
    }

    fn size(&self) -> Idx {
        self.size
    }

    fn indices(&self) -> impl Iterator<Item = Idx> {
        self.size.indices_below()
    }

    fn enumerate(&self) -> impl Iterator<Item = (Idx, &T)> {
        self.indices().map(|idx| (idx, &self[idx]))
    }

    fn contains_idx(&self, idx: Idx) -> bool {
        (0..self.size.x as isize).contains(&idx.x) && (0..self.size.y as isize).contains(&idx.y)
    }

    fn try_get(&self, idx: Idx) -> Option<&T> {
        if self.contains_idx(idx) {
            Some(&self.cells[self.cell_idx(idx)])
        } else {
            None
        }
    }

    fn cell_idx(&self, idx: Idx) -> usize {
        (self.size.x * idx.y + idx.x) as usize
    }
}

impl<T> Index<Idx> for Grid<T> {
    type Output = T;

    fn index(&self, idx: Idx) -> &Self::Output {
        &self.cells[self.cell_idx(idx)]
    }
}

impl<T> IndexMut<Idx> for Grid<T> {
    fn index_mut(&mut self, idx: Idx) -> &mut Self::Output {
        let cell_idx = self.cell_idx(idx);
        &mut self.cells[cell_idx]
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.size.y {
            if y > 0 {
                write!(f, "\n")?;
            }
            for x in 0..self.size.x {
                write!(f, "{}", self[Idx { x, y }])?;
            }
        }
        Ok(())
    }
}

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
