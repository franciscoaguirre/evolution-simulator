use std::time::Duration;

use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_rapier3d::na::{ArrayStorage, Const, Matrix};
use bevy_rapier3d::prelude::*;

use crate::genetic_algorithm::muscle_phenotype::MusclePhenotype;
use crate::prefabs::node;

pub struct Muscle {
    contracted_time: f32,
    extended_time: f32,
    contracted_length: f32,
    extended_length: f32,
    strength: f32,
    nodes: (Entity, Entity),
    timer: Timer,
    contracting: bool,
}

impl Muscle {
    fn from_phenotype(muscle_phenotype: &MusclePhenotype, nodes: &Vec<Entity>) -> Muscle {
        Muscle {
            nodes: (
                nodes[muscle_phenotype.nodes.0],
                nodes[muscle_phenotype.nodes.1],
            ),
            strength: muscle_phenotype.strength,
            extended_length: muscle_phenotype.extended_length,
            contracted_length: muscle_phenotype.contracted_length,
            extended_time: muscle_phenotype.extended_time,
            contracted_time: muscle_phenotype.contracted_time,
            timer: Timer::from_seconds(0.0, false),
            contracting: false,
        }
    }
}

pub fn create_muscle(
    parent: &mut ChildBuilder,
    muscle_phenotype: &MusclePhenotype,
    nodes: &Vec<Entity>,
) {
    parent
        .spawn()
        .insert(Muscle::from_phenotype(muscle_phenotype, nodes));
}

pub struct MusclePlugin;

impl Plugin for MusclePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(DebugLinesPlugin)
            .add_system(draw_muscles.system())
            .add_system(apply_forces.system());
    }
}

fn draw_muscles(
    muscles: Query<&Muscle>,
    nodes: Query<&ColliderPosition, With<node::Node>>,
    mut lines: ResMut<DebugLines>,
) {
    for muscle in muscles.iter() {
        let start = nodes.get(muscle.nodes.0).unwrap().0.translation.vector;
        let end = nodes.get(muscle.nodes.1).unwrap().0.translation.vector;
        lines.line(start.into(), end.into(), 0.0);
    }
}

type VectorMatrix =
    Matrix<f32, Const<3_usize>, Const<1_usize>, ArrayStorage<f32, 3_usize, 1_usize>>;

fn get_node_position(
    node: Entity,
    node_positions: &Query<&RigidBodyPosition, With<node::Node>>,
) -> VectorMatrix {
    node_positions
        .get(node)
        .unwrap()
        .position
        .translation
        .vector
}

fn apply_forces(
    time: Res<Time>,
    mut muscles: Query<&mut Muscle>,
    nodes: Query<&node::Node>,
    node_positions: Query<&RigidBodyPosition, With<node::Node>>,
    mut node_velocities: Query<&mut RigidBodyVelocity, With<node::Node>>,
    rb_mprops: Query<&RigidBodyMassProps>,
) {
    for mut muscle in muscles.iter_mut() {
        let first_node_position = get_node_position(muscle.nodes.0, &node_positions);
        let second_node_position = get_node_position(muscle.nodes.1, &node_positions);
        let first_to_second_direction = (second_node_position - first_node_position).normalize();
        let second_to_first_direction = (first_node_position - second_node_position).normalize();
        let muscle_length = (second_node_position - first_node_position).norm();

        let first_node_friction = nodes.get(muscle.nodes.0).unwrap().friction;
        let second_node_friction = nodes.get(muscle.nodes.1).unwrap().friction;

        let first_node_rb_mprops = rb_mprops.get(muscle.nodes.0).unwrap();
        let second_node_rb_mprops = rb_mprops.get(muscle.nodes.1).unwrap();

        let impulse_strength_first = muscle.strength * (1.0 / first_node_friction);
        let impulse_strength_second = muscle.strength * (1.0 / second_node_friction);

        if muscle.contracting && muscle_length <= muscle.contracted_length {
            let mut first_node_velocity = node_velocities.get_mut(muscle.nodes.0).unwrap();
            first_node_velocity.apply_impulse(
                first_node_rb_mprops,
                -first_to_second_direction * impulse_strength_first * time.delta_seconds(),
            );
            let mut second_node_velocity = node_velocities.get_mut(muscle.nodes.1).unwrap();
            second_node_velocity.apply_impulse(
                second_node_rb_mprops,
                -second_to_first_direction * impulse_strength_second * time.delta_seconds(),
            );
        } else if !muscle.contracting && muscle_length >= muscle.extended_length {
            let mut first_node_velocity = node_velocities.get_mut(muscle.nodes.0).unwrap();
            first_node_velocity.apply_impulse(
                first_node_rb_mprops,
                -second_to_first_direction * impulse_strength_first * time.delta_seconds(),
            );

            let mut second_node_velocity = node_velocities.get_mut(muscle.nodes.1).unwrap();
            second_node_velocity.apply_impulse(
                second_node_rb_mprops,
                -first_to_second_direction * impulse_strength_second * time.delta_seconds(),
            );
        }

        if muscle.timer.tick(time.delta()).just_finished() {
            let mut first_node_velocity = node_velocities.get_mut(muscle.nodes.0).unwrap();
            if muscle.contracting && muscle_length > muscle.contracted_length {
                first_node_velocity.apply_impulse(
                    first_node_rb_mprops,
                    first_to_second_direction * impulse_strength_first,
                );
            } else if !muscle.contracting && muscle_length < muscle.extended_length {
                first_node_velocity.apply_impulse(
                    first_node_rb_mprops,
                    second_to_first_direction * impulse_strength_first,
                );
            }

            let mut second_node_velocity = node_velocities.get_mut(muscle.nodes.1).unwrap();
            if muscle.contracting && muscle_length > muscle.contracted_length {
                second_node_velocity.apply_impulse(
                    second_node_rb_mprops,
                    second_to_first_direction * impulse_strength_second,
                );
            } else if !muscle.contracting && muscle_length < muscle.extended_length {
                second_node_velocity.apply_impulse(
                    second_node_rb_mprops,
                    first_to_second_direction * impulse_strength_second,
                );
            }

            muscle.contracting = !muscle.contracting;
            let timer_time = if muscle.contracting {
                muscle.contracted_time
            } else {
                muscle.extended_time
            };
            muscle
                .timer
                .set_duration(Duration::from_secs_f32(timer_time));
            muscle.timer.reset();
        }
    }
}
