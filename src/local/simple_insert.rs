use arrayvec::ArrayVec;
use cgmath::*;
use tribles::fucid;

use super::World;

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

pub struct Benchmark {
}

impl Benchmark {
    pub fn new() -> Self {
        Benchmark {}
    }

    pub fn run(&mut self) {
        let mut world = Box::new(World::new());

        let tf = world.new_component::<Transform>(fucid());
        let mut tf = tf.write().unwrap();
        let pos = world.new_component::<Position>(fucid());
        let mut pos = pos.write().unwrap();
        let rot = world.new_component::<Rotation>(fucid());
        let mut rot = rot.write().unwrap();
        let vel = world.new_component::<Velocity>(fucid());
        let mut vel = vel.write().unwrap();
        
        for _ in 0..1_000_000 {
            let entity = world.new_entity();
            entity.add_component(&mut tf, Transform(Matrix4::<f32>::from_scale(1.0)));
            entity.add_component(&mut pos, Position(Vector3::unit_x()));
            entity.add_component(&mut rot, Rotation(Vector3::unit_x()));
            entity.add_component(&mut vel, Velocity(Vector3::unit_x()));
        }

        let mut query = ArrayVec::new();
        query.push(pos.id);
        query.push(vel.id);
        
        world.new_query(&query);
    }
}
