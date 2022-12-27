use std::rc::Rc;

#[derive(Clone)]
pub struct AccumulateFilter<I: Iterator> {
    iter: I,
    filters: Vec<Rc<dyn Fn(<I as Iterator>::Item) -> bool>>,
}

impl<I: Iterator> AccumulateFilter<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            filters: vec![],
        }
    }

    pub fn add(&mut self, filter: Rc<dyn Fn(<I as Iterator>::Item) -> bool>) {
        self.filters.push(filter);
    }
}

impl<I: Iterator> Iterator for AccumulateFilter<I>
where
    <I as Iterator>::Item: Clone,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|e| {
            self.filters
                .iter()
                .map(|filter| filter(e.clone()))
                .find(|t| !t)
                .is_none()
        })
    }
}
