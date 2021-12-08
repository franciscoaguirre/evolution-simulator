use bevy::{core::FixedTimestep, prelude::*};

use super::resources::Config;

pub struct Velocity(pub Vec3);

pub struct PhysicsPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
pub enum PhysicsSystem {
    UpdateVelocity,
    ApplyVelocity,
}

const FIXED_TIME_STEP: f32 = 0.05;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
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

fn apply_gravity(mut velocities: Query<&mut Velocity>, config: Res<Config>) {
    let delta_time = FIXED_TIME_STEP * config.time_scale;

    let gravity = Vec3::new(0.0, -config.gravity, 0.0) * delta_time.powf(2.0);

    for mut velocity in velocities.iter_mut() {
        velocity.0 += gravity;
    }
}

// TODO: Make friction a parameter of nodes and apply here
fn apply_friction(mut velocity_nodes: Query<(&mut Velocity, &Transform)>, config: Res<Config>) {
    let delta_time = FIXED_TIME_STEP * config.time_scale;

    for (mut velocity, transform) in velocity_nodes.iter_mut() {
        if transform.translation.y > 0.01 {
            continue;
        }

        let friction = Vec3::new(-velocity.0.x * 0.9 * delta_time, 0.0, 0.0);
        velocity.0 += friction;
    }
}

fn apply_velocities(mut velocities: Query<(&mut Velocity, &mut Transform)>, config: Res<Config>) {
    let delta_time = FIXED_TIME_STEP * config.time_scale;

    for (mut velocity, mut transform) in velocities.iter_mut() {
        transform.translation.x += velocity.0.x * delta_time;
        transform.translation.y += velocity.0.y * delta_time;

        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
            velocity.0.y = 0.0;
        }
    }
}
