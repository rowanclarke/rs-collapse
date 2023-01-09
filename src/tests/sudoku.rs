use super::super::{
    collapse::{Collapse, FromCollapse},
    iter::AccumulateFilter,
};
use super::grid::Grid;
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
    ops::Range,
    rc::Rc,
};

#[test]
fn solve() {
    let mut solver = Solver::new();
    let initial = solver.get_state([
        [0, 0, 0, 0, 0, 4, 0, 5, 2],
        [6, 0, 0, 0, 0, 0, 0, 0, 0],
        [9, 0, 5, 0, 2, 0, 0, 3, 0],
        [0, 0, 0, 8, 0, 0, 7, 0, 0],
        [2, 0, 3, 0, 4, 0, 0, 9, 0],
        [0, 1, 0, 0, 0, 0, 0, 0, 0],
        [5, 0, 1, 0, 0, 7, 9, 0, 0],
        [0, 6, 0, 0, 5, 0, 0, 0, 0],
        [0, 4, 0, 0, 0, 0, 0, 1, 0],
    ]);
    let solution = solver.solve(Some(initial)).unwrap();
    assert_eq!(
        format!("\n{}", Sudoku::from_collapse(solution)),
        r"
1 3 8 9 7 4 6 5 2
6 2 4 1 3 5 8 7 9
9 7 5 6 2 8 4 3 1
4 9 6 8 1 3 7 2 5
2 5 3 7 4 6 1 9 8
8 1 7 5 9 2 3 6 4
5 8 1 2 6 7 9 4 3
3 6 9 4 5 1 2 8 7
7 4 2 3 8 9 5 1 6"
    );
}

struct Solver {
    grid: Grid,
}

impl Solver {
    pub fn new() -> Self {
        let grid = Grid::new(9, 9);
        Self { grid }
    }

    pub fn get_state(
        &self,
        state: [[<Self as Collapse>::Item; 9]; 9],
    ) -> BTreeMap<<Self as Collapse>::Coordinate, AccumulateFilter<<Self as Collapse>::State>> {
        let mut sudoku = self.get_initial();
        for coord in self.grid.clone().into_iter() {
            let item = state[coord.0][coord.1];
            if item != 0 {
                self.apply_single(coord, item, &mut sudoku);
            }
        }
        sudoku
    }
}

impl Collapse for Solver {
    type Item = u8;
    type Coordinate = (usize, usize);
    type State = <Range<Self::Item> as IntoIterator>::IntoIter;
    type Space = <Grid as IntoIterator>::IntoIter;

    fn update(
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

struct Sudoku {
    sudoku: [[u8; 9]; 9],
}

impl FromCollapse for Sudoku {
    type Collapser = Solver;

    fn from_collapse(
        solution: Vec<(
            <Self::Collapser as Collapse>::Coordinate,
            <Self::Collapser as Collapse>::Item,
        )>,
    ) -> Self {
        let mut sudoku = [[0; 9]; 9];
        solution
            .into_iter()
            .for_each(|(coord, item)| sudoku[coord.0][coord.1] = item);
        Self { sudoku }
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for i in 0..9 {
            for j in 0..9 {
                write!(f, "{}", self.sudoku[i][j])?;
                if j != 8 {
                    write!(f, " ")?;
                }
            }
            if i != 8 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
