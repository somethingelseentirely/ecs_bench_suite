use cgmath::*;

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
    world: Box<World>
}

impl Benchmark {
    pub fn new() -> Self {
        let mut world = Box::new(World::new());

        let tf = world.new_component::<Transform>();
        let mut tf = tf.write().unwrap();
        let pos = world.new_component::<Position>();
        let mut pos = pos.write().unwrap();
        let rot = world.new_component::<Rotation>();
        let mut rot = rot.write().unwrap();
        let vel = world.new_component::<Velocity>();
        let mut vel = vel.write().unwrap();
        
        for _ in 0..1_000_000 {
            let entity = world.new_entity();
            entity.add_component(&mut tf, Transform(Matrix4::<f32>::from_scale(1.0)));
            entity.add_component(&mut pos, Position(Vector3::unit_x()));
            entity.add_component(&mut rot, Rotation(Vector3::unit_x()));
            entity.add_component(&mut vel, Velocity(Vector3::unit_x()));
        }

        Benchmark {
            world
        }
    }

    pub fn run(&mut self) {
        /*
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
        */
    }
}
