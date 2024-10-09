use std::u32;

use arrayvec::ArrayVec;
use cgmath::*;
use rand::Rng;
use tribles::{fucid, genid, namespace::hex_literal::hex};

use super::{Id, World};

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
    world: Box<World>
}

const POS: Id = hex!("09D8E7A7E0A8B00C9E9823110D2842B6");
const VEL: Id = hex!("1FCC336CE90B1D9472A9B734586CA6AF");

impl Benchmark {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut world = Box::new(World::new());

        let tf = world.new_component::<Transform>(fucid());
        let mut tf = tf.write().unwrap();
        let pos = world.new_component::<Position>(POS);
        let mut pos = pos.write().unwrap();
        let rot = world.new_component::<Rotation>(fucid());
        let mut rot = rot.write().unwrap();
        let vel = world.new_component::<Velocity>(VEL);
        let mut vel = vel.write().unwrap();
        
        for _ in 0..1_000_000 {
            let entity = world.new_entity();
                entity.add_component(&mut tf, Transform(Matrix4::<f32>::from_scale(1.0)));
                entity.add_component(&mut pos, Position(Vector3::unit_x()));
                entity.add_component(&mut rot, Rotation(Vector3::unit_x()));
                entity.add_component(&mut vel, Velocity(Vector3::unit_x()));
        }


        let mut query = ArrayVec::new();
        query.push(POS);
        query.push(VEL);
        
        world.new_query(&query);

        Benchmark {
            world,
        }
    }

    pub fn run(&mut self) {
        let mut query = ArrayVec::new();
        query.push(POS);
        query.push(VEL);

        let pos = self.world.components.get(&POS).unwrap().clone();
        let mut pos: super::ColumnWriteGuard<'_, Position> = pos.write().unwrap();
        let vel = self.world.components.get(&VEL).unwrap().clone();
        let vel: super::ColumnReadGuard<'_, Velocity> = vel.read().unwrap();


        for q in self.world.query(&query).unwrap() {
            unsafe {
                pos.get_unchecked_mut(*q.get_unchecked(0)).inner.0 += vel.get_unchecked(*q.get_unchecked(1)).inner.0;
            }
        }
    }
}
