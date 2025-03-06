use std::collections::VecDeque;
use std::marker::PhantomData;
use crate::beans::Beans;
use crate::beans::beans_query::QueryData;

pub struct BeanQueryIter<'b, 's, D: QueryData> {
    fetch: D::Fetch<'b>,
    ids: VecDeque<String>,
    _marker: PhantomData<&'s ()>,
}

impl<'b, 's, D: QueryData> BeanQueryIter<'b, 's, D> {
    pub(crate) fn new(fetch: D::Fetch<'b>, ids: Vec<String>) -> Self {
        Self {
            fetch,
            ids: VecDeque::from(ids),
            _marker: Default::default(),
        }
    }
}

impl<'b, 's, D: QueryData> Iterator for BeanQueryIter<'b, 's, D> {
    type Item = D::Item<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.ids.pop_front()?;
        let fetched = D::fetch(
            &mut self.fetch,
            id
        );
        Some(fetched)
    }
}

