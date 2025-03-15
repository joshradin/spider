//! A list based Property

use crate::lazy::provider::{BoxProvider, Provider, ProviderExt, ProviderSource};
use crate::shared::Shared;
use parking_lot::lock_api::RwLock;
use std::collections::HashSet;
use std::sync::Arc;

enum ListPropertyItem<T> {
    Just(T),
    Provider(BoxProvider<T>),
    JustIterator(Box<dyn Fn() -> Box<dyn Iterator<Item = T> + Send + Sync> + Send + Sync>),
    ProviderIterator(
        Box<dyn Fn() -> Box<dyn Iterator<Item = BoxProvider<T>> + Send + Sync> + Send + Sync>,
    ),
    IteratorProvider(BoxProvider<Box<dyn Iterator<Item = T> + Send + Sync>>),
}

/// A property which provides an ordered list
pub struct ListProperty<T> {
    shared: Shared<Vec<ListPropertyItem<T>>>,
}

impl<T: Send + Sync + 'static> ListProperty<T> {
    pub(crate) fn new() -> Self
    where
        T: Clone,
    {
        Self {
            shared: Arc::new(RwLock::new(vec![])),
        }
    }

    /// Sets the value
    pub fn set<I>(&mut self, i: I)
    where
        I: IntoIterator<Item = T, IntoIter: Send + Sync + 'static> + Clone + Send + Sync + 'static,
    {
        let mut guard = self.shared.write();
        guard.clear();
        guard.push(ListPropertyItem::JustIterator(Box::new(move || {
            Box::new(i.clone().into_iter())
        })));
    }

    /// Sets the value from a provider
    pub fn set_from<I, P>(&mut self, p: P)
    where
        I: IntoIterator<Item = T, IntoIter: Send + Sync + 'static> + Send + Sync + 'static,
        P: Provider<I>,
        T: 'static,
    {
        let mut guard = self.shared.write();
        guard.clear();
        guard.push(ListPropertyItem::IteratorProvider(BoxProvider::new(p.map(
            |iterator| {
                let x = iterator.into_iter();
                Box::new(x) as Box<dyn Iterator<Item = T> + Send + Sync>
            },
        ))));
    }

    /// Add a value
    pub fn add(&mut self, v: T) {
        let mut guard = self.shared.write();
        guard.push(ListPropertyItem::Just(v));
    }

    /// Add a value from a provider
    pub fn add_from<P: Provider<T>>(&mut self, provider: &P)
    where
        T: 'static,
    {
        let mut guard = self.shared.write();
        guard.push(ListPropertyItem::Provider(BoxProvider::new(
            provider.clone(),
        )));
    }

    /// add all values
    pub fn add_all<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = T, IntoIter: Send + Sync + 'static>,
        I: Send + Sync + Clone + 'static,
    {
        let mut guard = self.shared.write();
        guard.push(ListPropertyItem::JustIterator(Box::new(move || {
            Box::new(items.clone().into_iter())
        })));
    }

    /// Add all values from a provider of an iterator
    pub fn add_all_from<
        I: IntoIterator<Item = T, IntoIter: Send + Sync + 'static> + Send + Sync + 'static,
        P: Provider<I>,
    >(
        &mut self,
        items: &P,
    )
        where T: Send + Sync + 'static,
    {
        let mut guard = self.shared.write();
        let provider = items.clone();
        let iterator_provider: BoxProvider<Box<dyn Iterator<Item = T> + Send + Sync>> = BoxProvider::new(provider.map(|i| {
            let i = i.into_iter();
            Box::new(i) as Box<dyn Iterator<Item=T> + Send + Sync>
        }));
        guard.push(
            ListPropertyItem::IteratorProvider(iterator_provider)
        );
    }
}

impl<'a, T: Send + Sync + 'static, P: Provider<T>> Extend<&'a P> for ListProperty<T> {
    fn extend<I: IntoIterator<Item = &'a P>>(&mut self, iter: I) {
        let mut guard = self.shared.write();
        let providers = iter.into_iter()
            .map(|p| p.clone())
            .map(|p| BoxProvider::new(p))
            .collect::<Vec<_>>();
        guard.push(
            ListPropertyItem::ProviderIterator(Box::new(move || { Box::new(providers.clone().into_iter()) }))
        );
    }
}

impl<T: Send + Sync + 'static> Clone for ListProperty<T> {
    fn clone(&self) -> Self {
        Self {
            shared: self.shared.clone(),
        }
    }
}

impl<T: Clone + Send + Sync + 'static> Provider<Vec<T>> for ListProperty<T> {
    fn try_get(&self) -> Option<Vec<T>> {
        let read = self.shared.read();
        let mut ret = vec![];
        for v in &*read {
            match v {
                ListPropertyItem::Just(t) => {
                    ret.push(t.clone());
                }
                ListPropertyItem::Provider(p) => {
                    ret.push(p.try_get()?);
                }
                ListPropertyItem::JustIterator(iterator) => {
                    for item in iterator() {
                        ret.push(item);
                    }
                }
                ListPropertyItem::ProviderIterator(iterator) => {
                    for item in iterator() {
                        ret.push(item.try_get()?);
                    }
                }
                ListPropertyItem::IteratorProvider(p) => {
                    for item in p.try_get()? {
                        ret.push(item);
                    }
                }
            }
        }
        Some(ret)
    }

    fn sources(&self) -> HashSet<ProviderSource> {
        let read = self.shared.read();
        let mut ret = HashSet::new();
        for v in &*read {
            match v {
                ListPropertyItem::Provider(p) => {
                    ret.extend(p.sources());
                }
                ListPropertyItem::ProviderIterator(iterator) => {
                    for item in iterator() {
                        ret.extend(item.sources());
                    }
                }
                ListPropertyItem::IteratorProvider(p) => {
                    ret.extend(p.sources());
                }
                _ => {}
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::lazy::provider::collections::ListProperty;
    use crate::lazy::provider::{Provider, ProviderFactory};

    #[test]
    fn test_list_property() {
        let mut property = ListProperty::<i32>::new();
        assert_eq!(property.get().len(), 0);
    }

    #[test]
    fn test_list_property_add() {
        let mut property = ListProperty::<i32>::new();
        property.add(1);
        assert_eq!(property.get().len(), 1);
    }

    #[test]
    fn test_list_property_add_all() {
        let mut property = ListProperty::<i32>::new();
        property.add_all([1, 2, 3]);
        assert_eq!(property.get().len(), 3);
    }

    #[test]
    fn test_list_property_add_all_from() {
        let provider_factory = ProviderFactory::new();
        let mut property = ListProperty::<i32>::new();
        property.add_all_from(&provider_factory.provider(|| [1, 2, 3]));
        property.add_all_from(&provider_factory.provider(|| [4, 5, 6]));
        let vec = property.get();
        assert_eq!(vec.len(), 6);
        assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_list_property_extend() {
        let provider_factory = ProviderFactory::new();
        let mut property = ListProperty::<i32>::new();
        property.extend([
            &provider_factory.just(1),
            &provider_factory.just(2),
            &provider_factory.just(3),
        ]);
        property.add_all_from(&provider_factory.provider(|| [4, 5, 6]));
        let vec = property.get();
        assert_eq!(vec.len(), 6);
        assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    }
}
