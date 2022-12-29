use super::AccumulateFilter;
use std::collections::BTreeMap;
use std::rc::Rc;

pub trait FromCollapse {
    type Collapser: Collapse;

    fn from_collapse(
        solution: Vec<(
            <Self::Collapser as Collapse>::Coordinate,
            <Self::Collapser as Collapse>::Item,
        )>,
    ) -> Self;
}

pub trait Collapse {
    type Item: Clone + PartialEq;
    type Coordinate: Ord + Clone + PartialEq;
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

    fn add_restriction(&self) {}

    fn solve(
        &mut self,
        initial: Option<BTreeMap<Self::Coordinate, AccumulateFilter<Self::State>>>,
    ) -> Result<Vec<(Self::Coordinate, Self::Item)>, NoSolution> {
        let coords = self.get_coords();
        let state = initial.unwrap_or_else(|| self.get_initial());
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
        let mut updater = self.update(coord.clone(), item);
        while let Some((coords, filter)) = updater.next() {
            let filter = Rc::new(filter);
            coords
                .filter(|c| c != &coord)
                .for_each(|coord| state.get_mut(&coord).unwrap().add(Rc::clone(&filter)));
        }
    }

    fn apply_single(
        &self,
        coord: Self::Coordinate,
        item: Self::Item,
        state: &mut BTreeMap<Self::Coordinate, AccumulateFilter<Self::State>>,
    ) where
        Self::Item: 'static,
    {
        let ic = item.clone();
        state
            .get_mut(&coord)
            .unwrap()
            .add(Rc::new(move |i| i == ic));
        self.apply_update(coord, item, state);
    }
}

#[derive(Debug)]
pub struct NoSolution;
