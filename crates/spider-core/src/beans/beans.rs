use crate::beans::cons::BeanConstructor;
use crate::beans::BeansParam;
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use snafu::Snafu;
use static_assertions::assert_impl_all;
use std::any;
use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::iter::empty;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// Stores beans for querying and adding
#[derive(Debug)]
pub struct Beans {
    beans: HashMap<String, Bean>,
    type_id_to_id: HashMap<TypeId, Vec<String>>,
}

impl Beans {
    /// Creates a new empty beans
    pub(crate) fn new() -> Self {
        Self {
            beans: Default::default(),
            type_id_to_id: Default::default(),
        }
    }

    /// Try to get a bean ref
    pub fn get<T>(&self, id: impl AsRef<str>) -> Result<BeanRef<T>>
    where
        T: Send + Sync + 'static,
    {
        let id = id.as_ref();
        let Some(bean) = self.beans.get(id) else {
            return NotFoundSnafu {
                name: id.to_string(),
            }
            .fail();
        };

        if bean.type_id != TypeId::of::<T>() {
            return WrongTypeSnafu {
                name: id.to_string(),
                expected: type_name::<T>(),
                actual: bean.type_name,
            }
            .fail();
        }

        let guard = bean.object.read();
        Ok(BeanRef {
            object: guard,
            _ty: PhantomData,
        })
    }

    /// Try to get a bean mutable reference
    pub fn get_mut<T>(&self, id: impl AsRef<str>) -> Result<BeanMut<T>>
    where
        T: Send + Sync + 'static,
    {
        let id = id.as_ref();
        let Some(bean) = self.beans.get(id) else {
            return NotFoundSnafu {
                name: id.to_string(),
            }
            .fail();
        };

        if bean.type_id != TypeId::of::<T>() {
            return WrongTypeSnafu {
                name: id.to_string(),
                expected: type_name::<T>(),
                actual: bean.type_name,
            }
            .fail();
        }

        let guard = bean.object.write();
        Ok(BeanMut {
            object: guard,
            _ty: PhantomData,
        })
    }

    /// gets all ids
    pub fn get_ids(&self) -> Vec<String> {
        self.beans.keys().cloned().collect()
    }

    /// Gets all ids of a given type
    pub fn get_ids_of<T>(&self) -> Vec<String>
    where
        T: Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();
        let ids = self.type_id_to_id.get(&type_id).cloned();
        ids.unwrap_or_default()
    }

    /// Gets all beans of a given type
    pub fn get_all<T>(&self) -> BeanRefIter<T>
    where
        T: Send + Sync + 'static,
    {
        BeanRefIter {
            beans: self,
            ids: self
                .type_id_to_id
                .get(&TypeId::of::<T>())
                .map(|s| Box::new(s.iter()) as Box<dyn Iterator<Item = _>>)
                .unwrap_or_else(|| Box::new(empty())),
            _ty: Default::default(),
        }
    }

    /// Gets all mutable beans of a given type
    pub fn get_all_mut<T>(&self) -> BeanMutIter<T>
    where
        T: Send + Sync + 'static,
    {
        BeanMutIter {
            beans: self,
            ids: self
                .type_id_to_id
                .get(&TypeId::of::<T>())
                .map(|s| Box::new(s.iter()) as Box<dyn Iterator<Item = _>>)
                .unwrap_or_else(|| Box::new(empty())),
            _ty: Default::default(),
        }
    }

    /// Insert a bean, failing if a bean with the given name already exists
    pub fn insert<T: Send + Sync + 'static>(
        &mut self,
        id: impl AsRef<str>,
        value: T,
    ) -> Result<()> {
        let id = id.as_ref();
        if self.beans.contains_key(id) {
            return AlreadyExistsSnafu {
                name: id.to_string(),
            }
            .fail();
        };

        let bean = Bean::new(value);
        self.beans.insert(id.to_string(), bean);
        self.type_id_to_id
            .entry(TypeId::of::<T>())
            .or_default()
            .push(id.to_string());

        Ok(())
    }

    /// Create a bean and insert
    pub fn init<T, Marker>(
        &mut self,
        name: impl AsRef<str>,
        cons: impl BeanConstructor<Marker, Out = T>,
    ) -> Result<()>
    where
        T: Send + Sync + 'static,
    {
        let cons = self.create(cons)?;
        self.insert(name, cons)
    }

    /// Create a value from some beans
    pub fn create<T, Marker>(&mut self, cons: impl BeanConstructor<Marker, Out = T>) -> Result<T>
    where
        T: Send + Sync + 'static,
    {
        self._create(cons)
    }

    fn _create<T, Marker, P: BeansParam>(
        &mut self,
        mut cons: impl BeanConstructor<Marker, Params=P, Out=T>,
    ) -> Result<T> {
        let mut state = P::init_state(self);
        let params = P::get_param(&mut state, self)?;

        cons.build(params)
    }
}

#[derive(Debug, Clone)]
struct Bean {
    type_id: TypeId,
    type_name: &'static str,
    object: Arc<RwLock<Box<dyn Any + Send + Sync>>>,
}

impl Bean {
    fn new<T: Send + Sync + 'static>(value: T) -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            type_name: any::type_name_of_val(&value),
            object: Arc::new(RwLock::new(Box::new(value) as Box<dyn Any + Send + Sync>)),
        }
    }
}

/// A reference to a bean
pub struct BeanRef<'a, T> {
    object: RwLockReadGuard<'a, Box<dyn Any + Send + Sync>>,
    _ty: PhantomData<fn() -> &'a T>,
}


impl<'a, T: Debug + 'static> Debug for BeanRef<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.get(), f)
    }
}

impl<'a, T: 'static> Deref for BeanRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, T: 'static> BeanRef<'a, T> {
    pub fn get(&self) -> &T {
        self.object.downcast_ref::<T>().unwrap()
    }
}

/// A mutable reference to a bean
pub struct BeanMut<'a, T> {
    object: RwLockWriteGuard<'a, Box<dyn Any + Send + Sync>>,
    _ty: PhantomData<fn() -> &'a T>,
}

impl<'a, T: Debug + 'static> Debug for BeanMut<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.get(), f)
    }
}

impl<'a, T: 'static> BeanMut<'a, T> {
    pub fn get(&self) -> &T {
        self.object.downcast_ref::<T>().unwrap()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.object.downcast_mut::<T>().unwrap()
    }
}

impl<'a, T: 'static> Deref for BeanMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, T: 'static> DerefMut for BeanMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

/// An error occurred using beans
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum BeanError {
    #[snafu(display("bean with id {name} already exists"))]
    AlreadyExists {
        name: String,
    },
    #[snafu(display("bean with id {name} not found"))]
    NotFound {
        name: String,
    },
    #[snafu(display("bean with id {name} has type {actual}, but {expected} was requested"))]
    WrongType {
        name: String,
        expected: &'static str,
        actual: &'static str,
    },
    Custom {
        error: Box<dyn Error + Send + Sync>,
    },
}

pub type Result<T> = std::result::Result<T, BeanError>;
pub type BeanResult<T> = std::result::Result<T, BeanError>;

pub struct BeanRefIter<'a, T> {
    beans: &'a Beans,
    ids: Box<dyn Iterator<Item = &'a String> + 'a>,
    _ty: PhantomData<fn() -> &'a T>,
}

impl<'a, T: Send + Sync + 'static> Iterator for BeanRefIter<'a, T> {
    type Item = BeanRef<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let name = self.ids.next()?;
        Some(self.beans.get(name).expect("should not fail"))
    }
}

assert_impl_all!(BeanRefIter<'static, ()>: Iterator<Item=BeanRef<'static, ()>>);

pub struct BeanMutIter<'a, T> {
    beans: &'a Beans,
    ids: Box<dyn Iterator<Item = &'a String> + 'a>,
    _ty: PhantomData<fn() -> &'a T>,
}

impl<'a, T: Send + Sync + 'static> Iterator for BeanMutIter<'a, T> {
    type Item = BeanMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let name = self.ids.next()?;
        Some(self.beans.get_mut(name).expect("should not fail"))
    }
}

assert_impl_all!(BeanMutIter<'static, ()>: Iterator<Item=BeanMut<'static, ()>>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::beans::Multi;
    use crate::lazy::providers::ProviderFactory;

    #[test]
    fn test_create_beans() {
        let mut beans = Beans::new();
        beans
            .insert("ProviderFactory", ProviderFactory::new())
            .expect("could not add");
        println!("beans: {:#?}", beans);
    }

    #[test]
    fn test_no_duplicate_beans() {
        let mut beans = Beans::new();
        beans
            .insert("ProviderFactory", ProviderFactory::new())
            .expect("could not add");
        beans
            .insert("ProviderFactory", ProviderFactory::new())
            .expect_err("could not add again");
        println!("beans: {:#?}", beans);
    }

    #[test]
    fn test_get_all() {
        let mut beans = Beans::new();
        beans.insert("a", 1_i32).unwrap();
        beans.insert("b", 2_i32).unwrap();
        assert_eq!(beans.get_all::<i32>().count(), 2);
        assert_eq!(beans.get_all::<u32>().count(), 0);
    }

    #[test]
    fn test_get_mut() {
        let mut beans = Beans::new();
        beans.insert("a", 1_i32).unwrap();
        assert_eq!(*beans.get::<i32>("a").unwrap(), 1);
        *beans.get_mut::<i32>("a").unwrap() = 2;
        assert_eq!(*beans.get::<i32>("a").unwrap(), 2);
    }

    #[test]
    fn test_create_from_beans() {
        let mut beans = Beans::new();

        fn immediate() -> BeanResult<i32> {
            Ok(1)
        }

        fn sum(all: Multi<&i32>) -> BeanResult<i32> {
            Ok(all.iter().map(|s| *s).sum())
        }

        beans.init("a", immediate).expect("could not create a");
        beans.init("b", immediate).expect("could not create b");
        beans.init("c", immediate).expect("could not create b");
        assert_eq!(*beans.get::<i32>("a").unwrap(), 1);
        let sum = beans.create(sum).expect("could not create sum");
        assert_eq!(sum, 3);

    }
}
