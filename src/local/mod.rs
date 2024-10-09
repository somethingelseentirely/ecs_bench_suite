use std::any::Any;
use std::collections::BTreeSet;
use std::ops::{Deref, DerefMut};
use std::sync::{atomic, Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{cmp::Ordering, collections::BTreeMap};

use std::borrow::Borrow;

use arrayvec::ArrayVec;
use rand::seq::index;
use tribles::fucid;

pub mod simple_insert;
pub mod simple_iter;

pub type Id = [u8; 16];

pub struct Row<T> {
    pub entity: Id,
    pub inner: T
}

impl<T> Row<T> {
    fn new(entity: Id, inner: T) -> Self {
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

const COMPONENT_LIMIT: usize = 16;

#[derive(Clone)]
pub struct Entity {
    pub id: Id,
    pub component_id: ArrayVec<Id, COMPONENT_LIMIT>,
    pub component_index: ArrayVec<usize, COMPONENT_LIMIT>
}

impl Entity {
    fn new(id: Id) -> Self {
        Self {
            id,
            component_id: ArrayVec::new(),
            component_index: ArrayVec::new()
        }
    }

    fn add_component<T: Send + Sync + 'static>(&mut self, column: &mut ColumnWriteGuard<T>, component: T) {
        let index = column.len();
        column.push(Row::new(self.id, component));
        self.component_id.push(column.id);
        self.component_index.push(index);
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

#[derive(Clone)]
pub struct Column {
    id: Id,
    data: Arc<dyn ColumnData>
}

pub struct ColumnReadGuard<'a, T> {
    pub id: Id,
    guard: RwLockReadGuard<'a, Vec<Row<T>>>
}

impl<'a, T> Deref for ColumnReadGuard<'a, T> {
    type Target = Vec<Row<T>>;

    fn deref(&self) -> &Self::Target {
        self.guard.deref()
    }
}

pub struct ColumnWriteGuard<'a, T> {
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
            data: Arc::new(RwLock::new(Vec::<Row<T>>::new()))
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

const QUERY_LIMIT: usize = 8;

pub struct World {
    pub entities: Vec<Entity>,
    pub queries: BTreeMap<ArrayVec<Id, QUERY_LIMIT>, Vec<usize>>,
    pub components: BTreeSet<Column>
}

impl World {
    fn new() -> Self {
        Self {
            entities: Vec::new(),
            queries: BTreeMap::new(),
            components: BTreeSet::new()
        }
    }
}

impl World {
    fn new_entity(&mut self) -> &mut Entity {
        let id = fucid();
        let entity = Entity::new(id);
        self.entities.push(entity);
        self.entities.last_mut().expect("just pushed")
    }

    fn new_component<T: Send + Sync + 'static>(&mut self, id: Id) -> Column {
        let col = Column::new::<T>(id);
        self.components.insert(col.clone());
        col
    }

    fn new_query(&mut self, components: &ArrayVec<Id, QUERY_LIMIT>) {
        if let None = self.queries.get(components) {
            let vec = Vec::new();
            self.queries.insert(components.clone(), vec);
            let vec = self.queries.get_mut(components).expect("just inserted");
            for entity in self.entities.iter() {
                if components.iter().all(|component| entity.component_id.contains(component)) {
                    for component_id in components {
                        let indexindex = entity.component_id.iter().position(|id|id == component_id).expect("is a subset");
                        let index = entity.component_index[indexindex];
                        vec.push(index);
                    }
                }
            }
        }
    }

    fn query(&mut self, components: &ArrayVec<Id, QUERY_LIMIT>) -> Option<impl Iterator<Item = &[usize]>> {
        let indices = self.queries.get(components)?;
        Some(indices.chunks_exact(components.len()))
    }

    //fn prepare_queries(&self, world; &mut World) {}
}


/*
fn join_helper<K: Ord, V1, V2>(
    mut slice1: &[(K, V1)],
    mut slice2: &[(K, V2)],
    mut result: impl FnMut(&K, &V1, &V2),
) {
    while !slice1.is_empty() && !slice2.is_empty() {
        use std::cmp::Ordering;

        // If the keys match produce tuples, else advance the smaller key until they might.
        match slice1[0].0.cmp(&slice2[0].0) {
            Ordering::Less => {
                slice1 = gallop(slice1, |x| x.0 < slice2[0].0);
            }
            Ordering::Equal => {
                // Determine the number of matching keys in each slice.
                let count1 = slice1.iter().take_while(|x| x.0 == slice1[0].0).count();
                let count2 = slice2.iter().take_while(|x| x.0 == slice2[0].0).count();

                // Produce results from the cross-product of matches.
                for index1 in 0..count1 {
                    for s2 in slice2[..count2].iter() {
                        result(&slice1[0].0, &slice1[index1].1, &s2.1);
                    }
                }

                // Advance slices past this key.
                slice1 = &slice1[count1..];
                slice2 = &slice2[count2..];
            }
            Ordering::Greater => {
                slice2 = gallop(slice2, |x| x.0 < slice1[0].0);
            }
        }
    }
}

pub(crate) fn gallop<T>(mut slice: &[T], mut cmp: impl FnMut(&T) -> bool) -> &[T] {
    // if empty slice, or already >= element, return
    if !slice.is_empty() && cmp(&slice[0]) {
        let mut step = 1;
        while step < slice.len() && cmp(&slice[step]) {
            slice = &slice[step..];
            step <<= 1;
        }

        step >>= 1;
        while step > 0 {
            if step < slice.len() && cmp(&slice[step]) {
                slice = &slice[step..];
            }
            step >>= 1;
        }

        slice = &slice[1..]; // advance one, as we always stayed < value
    }

    slice
}
*/