use itertools::{Itertools, Product};
use std::{
    iter::{empty, once, repeat, zip},
    ops::Range,
};

#[derive(Clone)]
pub struct Grid(usize, usize);

impl Grid {
    pub fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    pub fn row(&self, x: usize) -> impl Iterator<Item = (usize, usize)> {
        zip(repeat(x), 0..9)
    }

    pub fn column(&self, y: usize) -> impl Iterator<Item = (usize, usize)> {
        zip(0..9, repeat(y))
    }

    pub fn block(
        &self,
        (x, y): (usize, usize),
        (w, h): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        let (l, t) = (w * (x / w), w * (y / h));
        let (r, b) = (l + w, t + h);
        (l..r).cartesian_product(t..b)
    }

    pub fn get(
        &self,
        (x, y): (usize, usize),
        (x_off, y_off): (isize, isize),
    ) -> impl Iterator<Item = (usize, usize)> {
        let x = (x as isize) + x_off;
        let y = (y as isize) + y_off;
        if x >= 0 && y >= 0 && x < (self.0 as isize) && y < (self.1 as isize) {
            return vec![(x as usize, y as usize)].into_iter();
        }
        vec![].into_iter()
    }
}

impl IntoIterator for Grid {
    type Item = (usize, usize);
    type IntoIter = Product<Range<usize>, Range<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        (0..self.0).cartesian_product(0..self.1)
    }
}
