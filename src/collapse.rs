use super::AccumulateFilter;
use std::collections::BTreeMap;
use std::rc::Rc;
/*
pub trait FromCollapse {
    type FromCol: for<'a> Collapse + 'a;

    fn from_collapse(collapse: &Self::FromCol) -> Self;
}*/

pub trait Collapse {
    type Item: Clone;
    type Coordinate: Ord + Clone;
    type State: Iterator<Item = Self::Item> + Clone;
    type Space: Iterator<Item = Self::Coordinate> + Clone;

    fn update(
        &self,
        coord: Self::Coordinate,
        item: Self::Item,
    ) -> impl Iterator<
        Item = (
            impl Iterator<Item = Self::Coordinate>,
            Rc<dyn Fn(Self::Item) -> bool>,
        ),
    >;
    fn get_coords(&self) -> Self::Space;
    fn get_initial(&self) -> BTreeMap<Self::Coordinate, AccumulateFilter<Self::State>>;

    fn solve(&mut self) -> Result<Vec<(Self::Coordinate, Self::Item)>, NoSolution> {
        let coords = self.get_coords();
        let state = self.get_initial();
        self.try_state(coords, state)
    }

    fn try_state(
        &self,
        mut coords: Self::Space,
        state: BTreeMap<Self::Coordinate, AccumulateFilter<Self::State>>,
    ) -> Result<Vec<(Self::Coordinate, Self::Item)>, NoSolution> {
        if let Some(coord) = coords.next() {
            let mut moves = state.get(&coord).unwrap().clone();
            while let Some(item) = moves.next() {
                let mut state = state.clone();
                self.apply_update(coord.clone(), item.clone(), &mut state);
                if let Ok(mut vec) = self.try_state(coords.clone(), state) {
                    vec.push((coord, item));
                    return Ok(vec);
                }
            }
            return Err(NoSolution);
        }
        Ok(vec![])
    }

    fn apply_update(
        &self,
        coord: Self::Coordinate,
        item: Self::Item,
        state: &mut BTreeMap<Self::Coordinate, AccumulateFilter<Self::State>>,
    ) {
        let mut updater = self.update(coord, item);
        while let Some((coords, filter)) = updater.next() {
            let filter = Rc::new(filter);
            coords.for_each(|coord| state.get_mut(&coord).unwrap().add(Rc::clone(&filter)));
        }
    }
}

#[derive(Debug)]
pub struct NoSolution;
