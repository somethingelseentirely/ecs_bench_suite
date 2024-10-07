use std::collections::HashMap;

use cgmath::*;
use tribles::fucid;

use super::{Row, PushIndexed};

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
        let mut entities: Vec<([u8; 16], (usize, usize, usize, usize))> = Vec::new();

        let mut tf = Vec::new();
        let mut pos = Vec::new();
        let mut rot = Vec::new();
        let mut vel = Vec::new();
        
        (0..1_000_000).for_each(|_| {
            let entity = fucid();

            let tfi = tf.pushi(Row::new(entity, Transform(Matrix4::<f32>::from_scale(1.0))));
            let posi = pos.pushi(Row::new(entity, Position(Vector3::unit_x())));
            let roti = rot.pushi(Row::new(entity, Rotation(Vector3::unit_x())));
            let veli = vel.pushi(Row::new(entity, Velocity(Vector3::unit_x())));
            
            entities.push((entity, (tfi, posi, roti, veli)));
        });
    }
}