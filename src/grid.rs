use std::{
    fmt,
    ops::{Index, IndexMut},
};

#[derive(Copy, Clone, Debug)]
pub struct Idx {
    pub x: isize,
    pub y: isize,
}

impl Idx {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn offset(self, dx: isize, dy: isize) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn eight_neighbors(&self) -> impl Iterator<Item = Self> {
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

#[derive(Clone)]
pub struct Grid<T> {
    size: Idx,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from_rows(rows: impl Iterator<Item = impl Iterator<Item = T>>) -> Self {
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

    pub fn from_size_fn(size: Idx, f: impl FnMut(Idx) -> T) -> Self {
        Self {
            size,
            cells: size.indices_below().map(f).collect(),
        }
    }

    pub fn size(&self) -> Idx {
        self.size
    }

    pub fn indices(&self) -> impl Iterator<Item = Idx> {
        self.size.indices_below()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Idx, &T)> {
        self.indices().map(|idx| (idx, &self[idx]))
    }

    pub fn contains_idx(&self, idx: Idx) -> bool {
        (0..self.size.x as isize).contains(&idx.x) && (0..self.size.y as isize).contains(&idx.y)
    }

    pub fn try_get(&self, idx: Idx) -> Option<&T> {
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
