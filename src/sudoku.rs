use super::{AccumulateFilter, Collapse};
use itertools::{Itertools, Product};
use std::collections::BTreeMap;
use std::iter::{repeat, zip};
use std::ops::Range;
use std::rc::Rc;

pub struct Solver {
    grid: Grid,
}

impl Solver {
    pub fn new() -> Self {
        let grid = Grid::new(9, 9);
        Self { grid }
    }
}

impl Collapse for Solver {
    type Item = u32;
    type Coordinate = (usize, usize);
    type State = <Range<Self::Item> as IntoIterator>::IntoIter;
    type Space = <Grid as IntoIterator>::IntoIter;

    fn update<'a>(
        &self,
        coord: Self::Coordinate,
        item: Self::Item,
    ) -> impl Iterator<
        Item = (
            Box<dyn Iterator<Item = Self::Coordinate>>,
            Rc<dyn Fn(Self::Item) -> bool>,
        ),
    > {
        let remove: Rc<dyn Fn(Self::Item) -> bool> = Rc::new(move |e| e != item);
        let row: Box<dyn Iterator<Item = _>> = Box::new(self.grid.row(coord.0));
        let column: Box<dyn Iterator<Item = _>> = Box::new(self.grid.column(coord.1));
        let block: Box<dyn Iterator<Item = _>> = Box::new(self.grid.block(coord, (3, 3)));
        [
            (row, Rc::clone(&remove)),
            (column, Rc::clone(&remove)),
            (block, Rc::clone(&remove)),
        ]
        .into_iter()
    }

    fn get_coords(&self) -> Self::Space {
        self.grid.clone().into_iter()
    }

    fn get_initial(&self) -> BTreeMap<Self::Coordinate, AccumulateFilter<Self::State>> {
        let mut sudoku = BTreeMap::new();
        for coord in self.grid.clone().into_iter() {
            sudoku.insert(coord, AccumulateFilter::new(1..10));
        }
        sudoku
    }
}

#[derive(Clone)]
pub struct Grid(usize, usize);

impl Grid {
    fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    fn row(&self, x: usize) -> impl Iterator<Item = (usize, usize)> {
        zip(repeat(x), 0..9)
    }

    fn column(&self, y: usize) -> impl Iterator<Item = (usize, usize)> {
        zip(0..9, repeat(y))
    }

    fn block(
        &self,
        (x, y): (usize, usize),
        (w, h): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        let (l, t) = (w * (x / w), w * (y / h));
        let (r, b) = (l + w, t + h);
        (l..r).cartesian_product(t..b)
    }
}

impl IntoIterator for Grid {
    type Item = (usize, usize);
    type IntoIter = Product<Range<usize>, Range<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        (0..self.0).cartesian_product(0..self.1)
    }
}
/*
struct Sudoku {
    sudoku: [Unit; 81],
}

impl FromCollapse for Sudoku {
    type FromCol = Solver;

    fn from_collapse(collapse: &Self::FromCol) -> Self {
        unimplemented!()
    }
}
*/
