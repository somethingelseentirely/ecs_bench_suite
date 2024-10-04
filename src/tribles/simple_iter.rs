use std::{cell::Cell, collections::HashMap};

use cgmath::*;
use tribles::{and, find, id::fucid, query::ContainsConstraint, RawId};

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);
#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark{
    tf: HashMap<RawId, Cell<Transform>>,
    pos: HashMap<RawId, Cell<Position>>,
    rot: HashMap<RawId, Cell<Rotation>>,
    vel: HashMap<RawId, Cell<Velocity>>
}

impl Benchmark {
    pub fn new() -> Self {
        let mut tf = HashMap::new();
        let mut pos = HashMap::new();
        let mut rot = HashMap::new();
        let mut vel = HashMap::new();
        
        (0..1_000_000).for_each(|_| {
            let entity = fucid();
            tf.insert(entity, Transform(Matrix4::<f32>::from_angle_x(Rad(1.2))).into());
            pos.insert(entity, Position(Vector3::unit_x()).into());
            rot.insert(entity, Rotation(Vector3::unit_x()).into());
            vel.insert(entity, Velocity(Vector3::unit_x()).into());
        });

        Self{tf, pos, rot, vel}
    }

    pub fn run(&mut self) {

        for (entity,) in find!(ctx, (entity), and!(
            self.pos.has(entity),
            self.vel.has(entity))) {
                let entity: RawId = entity.try_unpack().unwrap();
                let pos_cell = self.pos.get(&entity).unwrap();
                let mut pos = pos_cell.get();
                let vel = self.vel.get(&entity).unwrap().get();
                pos.0 += vel.0;
                pos_cell.set(pos);
        }
    }
}
