use cgmath::*;
use shipyard::*;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let world = World::default();

        world.run(
            |mut entities: EntitiesViewMut,
             mut transforms: ViewMut<Transform>,
             mut positions: ViewMut<Position>,
             mut rotations: ViewMut<Rotation>,
             mut velocities: ViewMut<Velocity>| {
                for _ in 0..1_000_000 {
                    entities.add_entity(
                        (
                            &mut transforms,
                            &mut positions,
                            &mut rotations,
                            &mut velocities,
                        ),
                        (
                            Transform(Matrix4::from_scale(1.0)),
                            Position(Vector3::unit_x()),
                            Rotation(Vector3::unit_x()),
                            Velocity(Vector3::unit_x()),
                        ),
                    );
                }
            },
        ).unwrap();

        Self(world)
    }

    pub fn run(&mut self) {
        self.0.run(
            |velocities: View<Velocity>, mut positions: ViewMut<Position>| {
                (&velocities, &mut positions)
                    .iter()
                    .for_each(|(velocity, mut position)| {
                        position.0 += velocity.0;
                    })
            },
        ).unwrap();
    }
}
