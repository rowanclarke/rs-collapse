#![feature(
    type_alias_impl_trait,
    return_position_impl_trait_in_trait,
    impl_trait_projections
)]

mod collapse;
mod iter;
mod sudoku;

use collapse::{Collapse, FromCollapse};
use iter::AccumulateFilter;
use sudoku::{Solver, Sudoku};

fn main() {
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
    println!("{}", Sudoku::from_collapse(solution));
}
