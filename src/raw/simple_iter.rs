use std::collections::HashMap;

use cgmath::*;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);
#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark{
    tf: HashMap<u64, Transform>,
    pos: HashMap<u64, Position>,
    rot: HashMap<u64, Rotation>,
    vel: HashMap<u64, Velocity>
}

impl Benchmark {
    pub fn new() -> Self {
        let mut tf = HashMap::new();
        let mut pos = HashMap::new();
        let mut rot = HashMap::new();
        let mut vel = HashMap::new();
        
        (0..1_000_000).for_each(|i: u64| {
            let entity = i;
            tf.insert(entity, Transform(Matrix4::<f32>::from_angle_x(Rad(1.2))));
            pos.insert(entity, Position(Vector3::unit_x()));
            rot.insert(entity, Rotation(Vector3::unit_x()));
            vel.insert(entity, Velocity(Vector3::unit_x()));
        });

        Self{tf, pos, rot, vel}
    }

    pub fn run(&mut self) {
        for (entity, position) in self.pos.iter_mut() {
            if let Some(velocity) = self.vel.get(entity) {
                position.0 += velocity.0;
            }
        }
    }
}
