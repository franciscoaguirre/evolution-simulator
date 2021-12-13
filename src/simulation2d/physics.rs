use bevy::{core::FixedTimestep, prelude::*};

use crate::config::CONFIG;

use super::node;

pub struct Velocity(pub Vec3);

pub struct PhysicsPlugin {
    gravity: f32,
    air_friction: f32,
}

impl PhysicsPlugin {
    pub fn new(gravity: f32, air_friction: f32) -> PhysicsPlugin {
        PhysicsPlugin {
            gravity,
            air_friction,
        }
    }
}

impl Default for PhysicsPlugin {
    fn default() -> Self {
        PhysicsPlugin {
            gravity: 10.0,
            air_friction: 8.0,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
pub enum PhysicsSystem {
    UpdateVelocity,
    ApplyVelocity,
}

struct Gravity(f32);
pub struct AirFriction(pub f32);

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Gravity(self.gravity))
            .insert_resource(AirFriction(self.air_friction))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(
                        CONFIG.fixed_time_step as f64 / CONFIG.time_scale as f64,
                    ))
                    .with_system(apply_gravity.system().label(PhysicsSystem::UpdateVelocity))
                    .with_system(apply_friction.system().label(PhysicsSystem::UpdateVelocity))
                    .with_system(
                        apply_velocities
                            .system()
                            .label(PhysicsSystem::ApplyVelocity)
                            .after(PhysicsSystem::UpdateVelocity),
                    ),
            );
    }
}

fn apply_gravity(mut velocities: Query<&mut Velocity>, gravity: Res<Gravity>) {
    let my_span = info_span!("system", name = "apply_gravity");
    let _guard = my_span.enter();
    let delta_time = CONFIG.fixed_time_step;

    let gravity = Vec3::new(0.0, -gravity.0, 0.0) * delta_time.powf(2.0);

    for mut velocity in velocities.iter_mut() {
        velocity.0 += gravity;
    }
}

fn apply_friction(mut velocity_nodes: Query<(&mut Velocity, &Transform, &node::Node)>) {
    let my_span = info_span!("system", name = "apply_friction");
    let _guard = my_span.enter();

    let delta_time = CONFIG.fixed_time_step;

    for (mut velocity, transform, node) in velocity_nodes.iter_mut() {
        if transform.translation.y > 0.01 {
            continue;
        }

        let friction = Vec3::new(-velocity.0.x * node.friction * delta_time, 0.0, 0.0);
        velocity.0 += friction;
    }
}

fn apply_velocities(mut velocities: Query<(&mut Velocity, &mut Transform)>) {
    let my_span = info_span!("system", name = "apply_velocities");
    let _guard = my_span.enter();

    let delta_time = CONFIG.fixed_time_step;

    for (mut velocity, mut transform) in velocities.iter_mut() {
        transform.translation.x += velocity.0.x * delta_time;
        transform.translation.y += velocity.0.y * delta_time;

        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
            velocity.0.y = 0.0;
        }
    }
}
