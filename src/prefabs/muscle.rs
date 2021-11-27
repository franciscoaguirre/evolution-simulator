use bevy::core::Stopwatch;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
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
        app.insert_resource(Stopwatch::new())
            .add_plugin(DebugLinesPlugin)
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

fn apply_forces(
    time: Res<Time>,
    mut stopwatch: ResMut<Stopwatch>,
    muscles: Query<&Muscle>,
    nodes: Query<&node::Node>,
    node_positions: Query<&RigidBodyPosition, With<node::Node>>,
    mut node_velocities: Query<&mut RigidBodyVelocity, With<node::Node>>,
) {
    stopwatch.tick(time.delta());

    for muscle in muscles.iter() {
        let should_contract = stopwatch.elapsed_secs() <= muscle.contracted_time;
        let first_node_position = node_positions
            .get(muscle.nodes.0)
            .unwrap()
            .position
            .translation
            .vector;
        let second_node_position = node_positions
            .get(muscle.nodes.1)
            .unwrap()
            .position
            .translation
            .vector;
        let first_to_second_direction = (second_node_position - first_node_position).normalize();
        let second_to_first_direction = (first_node_position - second_node_position).normalize();
        let muscle_length = (second_node_position - first_node_position).norm();

        let first_node_friction = nodes.get(muscle.nodes.0).unwrap().friction;
        let second_node_friction = nodes.get(muscle.nodes.1).unwrap().friction;

        let mut first_node_velocity = node_velocities.get_mut(muscle.nodes.0).unwrap();
        if should_contract && muscle_length > muscle.contracted_length {
            first_node_velocity.linvel = first_to_second_direction * muscle.strength * (1.0 / first_node_friction);
        } else if !should_contract && muscle_length < muscle.extended_length {
            first_node_velocity.linvel = second_to_first_direction * muscle.strength * (1.0 / first_node_friction);
        } else {
            first_node_velocity.linvel = Vec3::ZERO.into();
        }

        let mut second_node_velocity = node_velocities.get_mut(muscle.nodes.1).unwrap();
        if should_contract && muscle_length > muscle.contracted_length {
            second_node_velocity.linvel = second_to_first_direction * muscle.strength * (1.0 / second_node_friction);
        } else if !should_contract && muscle_length < muscle.extended_length {
            second_node_velocity.linvel = first_to_second_direction * muscle.strength * (1.0 / second_node_friction);
        } else {
            second_node_velocity.linvel = Vec3::ZERO.into();
        }
    }

    if stopwatch.elapsed_secs() >= 1.0 {
        stopwatch.reset();
    }
}
