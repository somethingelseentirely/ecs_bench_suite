use std::collections::HashMap;

use cgmath::*;
use tribles::id::fucid;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);
#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark;

impl Benchmark {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) {
        let mut tf = HashMap::new();
        let mut pos = HashMap::new();
        let mut rot = HashMap::new();
        let mut vel = HashMap::new();
        
        (0..1_000_000).for_each(|entity: u128| {
            let entity = entity.to_be_bytes();
            tf.insert(entity, Transform(Matrix4::<f32>::from_scale(1.0)));
            pos.insert(entity, Position(Vector3::unit_x()));
            rot.insert(entity, Rotation(Vector3::unit_x()));
            vel.insert(entity, Velocity(Vector3::unit_x()));
        });
    }
}
