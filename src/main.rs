#![feature(
    type_alias_impl_trait,
    return_position_impl_trait_in_trait,
    impl_trait_projections
)]

mod collapse;
mod iter;
mod sudoku;

use collapse::Collapse;
use iter::AccumulateFilter;
use std::rc::Rc;
use sudoku::Solver;

fn main() {
    let mut s = Solver::new();
    let a = s.solve();
    let mut b = [[0; 9]; 9];
    a.unwrap()
        .into_iter()
        .for_each(|(coord, i)| b[coord.0][coord.1] = i);
    for i in 0..9 {
        for j in 0..9 {
            print!("{} ", b[i][j]);
        }
        println!();
    }
}
