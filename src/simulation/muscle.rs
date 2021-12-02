use bevy::core::Stopwatch;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_rapier3d::na::{ArrayStorage, Const, Matrix};
use bevy_rapier3d::prelude::*;

use super::node;
use crate::genetic_algorithm::muscle_phenotype::MusclePhenotype;

pub struct Muscle {
    contracted_time: f32,
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
            .insert_resource(Timer::from_seconds(2.0, false))
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

type ColumnMatrix =
    Matrix<f32, Const<3_usize>, Const<1_usize>, ArrayStorage<f32, 3_usize, 1_usize>>;

fn get_node_position(
    node: Entity,
    node_positions: &Query<&ColliderPosition, With<node::Node>>,
) -> ColumnMatrix {
    node_positions.get(node).unwrap().0.translation.vector
}

fn apply_forces(
    time: Res<Time>,
    mut stopwatch: ResMut<Stopwatch>,
    muscles: Query<&Muscle>,
    nodes: Query<&node::Node>,
    node_positions: Query<&ColliderPosition, With<node::Node>>,
    mut node_velocities: Query<&mut RigidBodyVelocity, With<node::Node>>,
) {
    stopwatch.tick(time.delta());

    for muscle in muscles.iter() {
        let should_contract = stopwatch.elapsed_secs() <= muscle.contracted_time;
        let target_length = if should_contract {
            muscle.contracted_length
        } else {
            muscle.extended_length
        };

        let first_node_position = get_node_position(muscle.nodes.0, &node_positions);
        let second_node_position = get_node_position(muscle.nodes.1, &node_positions);
        let mut first_to_second_direction = (second_node_position - first_node_position).normalize();

        if first_node_position == second_node_position {
            first_to_second_direction = Vec3::new(0.0, 0.0, 1.0).into();
        }

        let second_to_first_direction = -first_to_second_direction;
        let muscle_length = (second_node_position - first_node_position).norm();

        let force = (1.0 - (muscle_length / target_length).powf(2.0))
            .max(-0.4)
            .min(0.4);

        let first_node_friction = nodes.get(muscle.nodes.0).unwrap().friction;
        let second_node_friction = nodes.get(muscle.nodes.1).unwrap().friction;

        let first_node_strength = muscle.strength * (1.0 / first_node_friction);
        let second_node_strength = muscle.strength * (1.0 / second_node_friction);

        let mut first_node_velocity = node_velocities.get_mut(muscle.nodes.0).unwrap();
        first_node_velocity.linvel +=
            second_to_first_direction * force * first_node_strength * time.delta_seconds();

        let mut second_node_velocity = node_velocities.get_mut(muscle.nodes.1).unwrap();
        second_node_velocity.linvel +=
            first_to_second_direction * force * second_node_strength * time.delta_seconds();
    }

    // TODO: Make this 10.0 a constant or a variable that's different for each creature
    if stopwatch.elapsed_secs() >= 1.0 {
        stopwatch.reset();
    }
}
