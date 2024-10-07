use std::{any::Any, collections::{BTreeMap, BTreeSet, HashMap, HashSet}, sync::{Arc, Mutex}};

use bevy_ecs::component;
use rand::thread_rng;
use rand::seq::SliceRandom;

use cgmath::*;
use tribles::fucid;

use super::{AnyColumn, Column, ColumnWriteGuard, Entity, Id, PushIndexed, Row};

#[derive(Copy, Clone)]
pub struct Transform(Matrix4<f32>);
#[derive(Copy, Clone)]
pub struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
pub struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
pub struct Velocity(Vector3<f32>);

pub struct Query {
    
}

pub struct World {
    pub entities: Vec<Entity>,
    pub queries: BTreeMap<BTreeSet<Id>, Vec<usize>>,
    pub components: BTreeSet<AnyColumn>
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
    fn prepare(&mut self, components: BTreeSet<Id>) {
        if let None = self.queries.get(&components) {
            let vec = Vec::new();
            self.queries.insert(components.clone(), vec);
            let vec = self.queries.get_mut(&components).expect("just inserted");
            for entity in self.entities.iter() {
                let entity_components: BTreeSet<Id> = entity.components.keys().copied().collect();
                if components.is_subset(&entity_components) {
                    for id in &components {
                        vec.push(*entity.components.get(id).expect("is a subset"));
                    }
                }
            }
        }
    }

    fn new_entity(&mut self) -> &mut Entity {
        let id = fucid();
        let entity = Entity::new(id);
        self.entities.push(entity);
        self.entities.last_mut().expect("just pushed")
    }

    fn new_component<T>(&mut self) -> &Column {
        let id = fucid();
        let col = Column::new(id);
        self.components.insert(col);
        self.components.get(&id)
    }
}

impl Entity {
    fn add_component<T: Send + Sync + 'static>(&mut self, column: &mut ColumnWriteGuard<T>, component: T) {
        assert!(!self.components.contains_key(&column.id), "components may only be added once to an entity");
        let index = column.len();
        column.push(Row::new(self.id, component));
        self.components.insert(column.id, index);
    }
}

pub struct Benchmark {
    world: World
}

impl Benchmark {
    pub fn new() -> Self {
        let world = World::new();

        let tf = world.new_component().write().unwrap();
        let pos = world.new_component().write().unwrap();
        let rot = world.new_component().write().unwrap();
        let vel = world.new_component().write().unwrap();
        
        (0..1_000_000).for_each(|_| {
            let entity = world.new_entity();
            entity.add_component(&mut tf, Transform(Matrix4::<f32>::from_scale(1.0)));
            entity.add_component(&mut pos, Position(Vector3::unit_x()));
            entity.add_component(&mut rot, Rotation(Vector3::unit_x()));
            entity.add_component(&mut vel, Velocity(Vector3::unit_x()));
        });

        Benchmark {
            world
        }
    }

    pub fn run(&mut self) {
        for entity in self.entities.iter() {
            match entity {
                Entity {pos: Some(posi),
                        vel: Some(veli),
                        ..} => {
                    self.pos[*posi].inner.0 += self.vel[*veli].inner.0;
                }
                _ => {}
            }
        }
    }
}
