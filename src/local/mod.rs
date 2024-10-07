use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{cmp::Ordering, collections::BTreeMap};

use std::borrow::Borrow;

pub mod simple_insert;
pub mod simple_iter;

pub type Id = [u8; 16];

pub struct Row<T> {
    pub entity: [u8; 16],
    pub inner: T
}

impl<T> Row<T> {
    fn new(entity: [u8; 16], inner: T) -> Self {
        Self {
            entity,
            inner
        }
    }
}

impl<T> Ord for Row<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.entity.cmp(&other.entity)
    }
}

impl<T> PartialOrd for Row<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Row<T> {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

impl<T> Eq for Row<T> {}

impl<T> Borrow<Id> for Row<T> {
    fn borrow(&self) -> &Id {
        &self.entity
    }
}

pub struct Entity {
    pub id: Id,
    pub components: BTreeMap<Id, usize>
}

impl  Entity {
    fn new(id: Id) -> Self {
        Self {
            id,
            components: BTreeMap::new()
        }
    }
}

impl Ord for Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Entity {}

impl Borrow<Id> for Entity {
    fn borrow(&self) -> &Id {
        &self.id
    }
}

trait ColumnData {
    fn as_any(&self) -> &(dyn Any + Sync + Send);
}

impl<T: Sync + Send + 'static> ColumnData for RwLock<Vec<Row<T>>> {
    fn as_any(&self) -> &(dyn Any + Sync + Send) {
        self
    }
}

struct Column {
    id: Id,
    data: Box<dyn ColumnData>
}

struct ColumnReadGuard<'a, T> {
    pub id: Id,
    guard: RwLockReadGuard<'a, Vec<Row<T>>>
}

impl<'a, T> Deref for ColumnReadGuard<'a, T> {
    type Target = Vec<Row<T>>;

    fn deref(&self) -> &Self::Target {
        self.guard.deref()
    }
}

struct ColumnWriteGuard<'a, T> {
    pub id: Id,
    guard: RwLockWriteGuard<'a, Vec<Row<T>>>
}

impl<'a, T> Deref for ColumnWriteGuard<'a, T> {
    type Target = Vec<Row<T>>;

    fn deref(&self) -> &Self::Target {
        self.guard.deref()
    }
}

impl<'a, T> DerefMut for ColumnWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.deref_mut()
    }
}

impl Column {
    pub fn new<T: Send + Sync + 'static>(id: Id) -> Self {
        Self {
            id,
            data: Box::new(RwLock::new(Vec::<Row<T>>::new()))
        }
    }

    pub fn read<'a, T>(&'a self) -> Option<ColumnReadGuard<'a, T>>
    where
        T: Send + Sync + 'static,
    {
        let data = &*self.data;
        let data = ColumnData::as_any(data);
        let lock = data.downcast_ref::<RwLock<Vec<Row<T>>>>()?;
        let guard = lock.read().ok()?;
        Some(ColumnReadGuard {
            id: self.id,
            guard
        })
    }

    pub fn write<'a, T>(&'a self) -> Option<ColumnWriteGuard<'a, T>>
    where
        T: Send + Sync + 'static,
    {
        let data = &*self.data;
        let data = ColumnData::as_any(data);
        let lock = data.downcast_ref::<RwLock<Vec<Row<T>>>>()?;
        let guard = lock.write().ok()?;
        Some(ColumnWriteGuard {
            id: self.id,
            guard
        })
    }
}

impl Ord for Column {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Column {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Column {}

impl Borrow<Id> for Column {
    fn borrow(&self) -> &Id {
        &self.id
    }
}