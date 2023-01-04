use super::{
    collapse::{Collapse, FromCollapse},
    grid::Grid,
    iter::AccumulateFilter,
};
use itertools::{Itertools, Product};
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
    ops::Range,
    rc::Rc,
};

pub struct Solver {
    grid: Grid,
}

impl Solver {
    pub fn new() -> Self {
        let grid = Grid::new(4, 4);
        Self { grid }
    }

    fn fit(
        Piece(piece, rotation): <Self as Collapse>::Item,
        direction: usize,
    ) -> Rc<dyn Fn(<Self as Collapse>::Item) -> bool> {
        Rc::new(move |Piece(p, r)| {
            PIECES[piece][(rotation + direction) % 4] == PIECES[p][(r + 2 + direction) % 4]
        })
    }
}

impl Collapse for Solver {
    type Item = Piece;
    type Coordinate = (usize, usize);
    type State = Board;
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
        let remove: Rc<dyn Fn(Self::Item) -> bool> = Rc::new(move |Piece(p, _)| p != item.0);
        let all: Box<dyn Iterator<Item = _>> = Box::new(self.grid.clone().into_iter());
        let mut update = vec![(all, Rc::clone(&remove))];
        let neighbors = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        for i in 0..4 {
            let neighbor: Box<dyn Iterator<Item = _>> =
                Box::new(self.grid.get(coord, neighbors[i]));
            update.push((neighbor, Self::fit(item, i)));
        }
        update.into_iter()
    }

    fn get_coords(&self) -> Self::Space {
        self.grid.clone().into_iter()
    }

    fn get_initial(&self) -> BTreeMap<Self::Coordinate, AccumulateFilter<Self::State>> {
        let mut puzzle = BTreeMap::new();
        for coord in self.grid.clone().into_iter() {
            puzzle.insert(
                coord,
                AccumulateFilter::new(Board((0..16).cartesian_product(0..4))),
            );
        }
        puzzle
    }
}

#[derive(Clone)]
pub struct Board(Product<Range<usize>, Range<usize>>);

impl Iterator for Board {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(p, r)| Piece(p, r))
    }
}

pub struct Puzzle {
    puzzle: [[Piece; 4]; 4],
}

impl FromCollapse for Puzzle {
    type Collapser = Solver;

    fn from_collapse(
        solution: Vec<(
            <Self::Collapser as Collapse>::Coordinate,
            <Self::Collapser as Collapse>::Item,
        )>,
    ) -> Self {
        let mut puzzle = [[Piece(0, 0); 4]; 4];
        solution
            .into_iter()
            .for_each(|(coord, item)| puzzle[coord.0][coord.1] = item);
        Self { puzzle }
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    write!(f, "{}", self.puzzle[k][i].get_line(j))?;
                    if k != 3 {
                        write!(f, " ")?;
                    }
                }
                if j != 3 {
                    writeln!(f)?;
                }
            }
            if i != 3 {
                write!(f, "\n\n")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Piece(usize, usize);

impl Piece {
    fn get_line(&self, i: usize) -> String {
        let mut sides = Vec::with_capacity(4);
        for j in 0..4 {
            sides.push(PIECES[self.0][(j + self.1) % 4]);
        }
        if i == 0 {
            format!(" {}{} ", sides[0].0, sides[0].1)
        } else if i == 1 {
            format!("{}  {}", sides[3].1, sides[1].0)
        } else if i == 2 {
            format!("{}  {}", sides[3].0, sides[1].1)
        } else {
            format!(" {}{} ", sides[2].1, sides[2].0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Collapse, FromCollapse, Puzzle, Solver};

    #[test]
    fn solve() {
        let mut solver = Solver::new();
        let solution = solver.solve(None).unwrap();
        assert_eq!(
            format!("{}", Puzzle::from_collapse(solution)),
            r" PW   TP   WT   RT 
R       W W  P P  T
   R R  W W  W W  P
 WP   TR   PT   RW 

 WP   TR   PT   RW 
W  T T  W W  R R  W
R  P P  T T  W W  P
 R    WR   TR   WT 

 R    WR   TR   WT 
P  W W  R R  W W   
W  P P  T T  T T  T
 PR   PT    W   T  

 PR   PT    W   T  
R  T T  W W  W W  R
T  W W  R R  R R  W
 PW   PR   WP   TP "
        );
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Link {
    None,
    Road,
    Track,
    Path,
    River,
}

impl Display for Link {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Link::None => " ",
                Link::Road => "R",
                Link::Track => "T",
                Link::Path => "P",
                Link::River => "W",
            }
        )
    }
}

#[derive(Copy, Clone)]
pub struct Side(pub Link, pub Link);

impl PartialEq<Self> for Side {
    fn eq(&self, rhs: &Self) -> bool {
        // Can they fit together?
        return self.0 == rhs.1 && self.1 == rhs.0;
    }
}

const PIECES: [[Side; 4]; 16] = [
    [
        // 1 (Yellow Fisherman)
        Side(Link::Track, Link::River),
        Side(Link::Path, Link::Track),
        Side(Link::Road, Link::River),
        Side(Link::Road, Link::Track),
    ],
    [
        // 2 (Milk Truck)
        Side(Link::Track, Link::Road),
        Side(Link::River, Link::Track),
        Side(Link::River, Link::None),
        Side(Link::Track, Link::Road),
    ],
    [
        // 3 (Red-Ore Train)
        Side(Link::Path, Link::River),
        Side(Link::Track, Link::Path),
        Side(Link::River, Link::River),
        Side(Link::River, Link::Track),
    ],
    [
        // 4 (Orange-Ore Train)
        Side(Link::Track, Link::River),
        Side(Link::River, Link::Track),
        Side(Link::None, Link::Track),
        Side(Link::None, Link::Track),
    ],
    [
        // 5 (Red Fisherman)
        Side(Link::None, Link::Road),
        Side(Link::Path, Link::River),
        Side(Link::None, Link::Road),
        Side(Link::Path, Link::River),
    ],
    [
        // 6 (Black-White-Tailed Dog)
        Side(Link::Track, Link::Path),
        Side(Link::None, Link::Road),
        Side(Link::Road, Link::River),
        Side(Link::River, Link::Path),
    ],
    [
        // 7 (Blue Car)
        Side(Link::Path, Link::Track),
        Side(Link::Road, Link::River),
        Side(Link::Track, Link::None),
        Side(Link::Road, Link::River),
    ],
    [
        // 8 (Apple Truck)
        Side(Link::River, Link::Road),
        Side(Link::River, Link::Path),
        Side(Link::Road, Link::Track),
        Side(Link::Track, Link::Path),
    ],
    [
        // 9 (Red Car)
        Side(Link::Road, Link::Track),
        Side(Link::Road, Link::None),
        Side(Link::Track, Link::Path),
        Side(Link::River, Link::River),
    ],
    [
        // 10 (White-Tailed Dog)
        Side(Link::River, Link::Path),
        Side(Link::Track, Link::River),
        Side(Link::River, Link::Road),
        Side(Link::Road, Link::River),
    ],
    [
        // 11 (Black-Tailed Dog)
        Side(Link::Track, Link::Path),
        Side(Link::Path, Link::River),
        Side(Link::River, Link::Road),
        Side(Link::Road, Link::Track),
    ],
    [
        // 12 (Yellow Boat)
        Side(Link::River, Link::Track),
        Side(Link::Road, Link::River),
        Side(Link::Path, Link::Track),
        Side(Link::Track, Link::Road),
    ],
    [
        // 13 (Sun Truck)
        Side(Link::Path, Link::River),
        Side(Link::Road, Link::River),
        Side(Link::None, Link::River),
        Side(Link::River, Link::Road),
    ],
    [
        // 14 (Yellow Car)
        Side(Link::Track, Link::River),
        Side(Link::River, Link::Path),
        Side(Link::Track, Link::Road),
        Side(Link::Path, Link::Road),
    ],
    [
        // 15 (Orange Boat)
        Side(Link::Road, Link::Path),
        Side(Link::River, Link::Path),
        Side(Link::Road, Link::None),
        Side(Link::River, Link::Path),
    ],
    [
        // 16 (Blue-Ore Train)
        Side(Link::Path, Link::Track),
        Side(Link::River, Link::Road),
        Side(Link::Road, Link::Path),
        Side(Link::River, Link::Track),
    ],
];
