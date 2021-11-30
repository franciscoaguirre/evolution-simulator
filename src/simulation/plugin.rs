use bevy::{core::Stopwatch, prelude::*};
use bevy_rapier3d::prelude::ColliderPosition;

use crate::genetic_algorithm::plugin::{StartEvaluatingEvent, POPULATION_SIZE};

use super::{
    creature::{create_creature, Creature},
    muscle::MusclePlugin,
    node,
};

pub struct SimulationPlugin;

#[derive(Default)]
struct CreaturesCreated(usize);

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(MusclePlugin)
            .insert_resource(CreaturesCreated::default())
            .insert_resource(Timer::from_seconds(2.0, false))
            .add_system(simulate.system())
            .add_system(evaluate_simulation.system());
    }
}

fn simulate(
    mut creatures_created: ResMut<CreaturesCreated>,
    mut commands: Commands,
    mut stopwatch: ResMut<Stopwatch>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut start_evaluating_event: EventReader<StartEvaluatingEvent>,
) {
    for event in start_evaluating_event.iter() {
        // TODO: Move position depending on amount spwaned
        create_creature(
            &mut commands,
            &mut meshes,
            &mut materials,
            event.chromosome.clone(),
            Vec3::default() + Vec3::new(1.0, 0.0, 0.0) * creatures_created.0 as f32,
        );

        creatures_created.0 += 1;
    }

    if creatures_created.0 == POPULATION_SIZE * 2 {
        stopwatch.reset();
    }
}

fn evaluate_simulation(
    mut creatures_created: ResMut<CreaturesCreated>,
    stopwatch: ResMut<Stopwatch>,
    mut creatures: Query<(Entity, &mut Creature)>,
    collider_node_positions: Query<(&ColliderPosition, &Parent), With<node::Node>>,
) {
    // Simulation has ended. We should declare all results
    if stopwatch.elapsed_secs() <= 30.0 {
        return;
    }

    for (entity, mut creature) in creatures.iter_mut() {
        // TODO: Evaluate initial position and check final position
        let mut position = Vec3::default();
        for (collider_position, parent) in collider_node_positions.iter() {
            if parent.0 != entity {
                continue;
            }

            position += collider_position.0.translation.vector.into();
        }

        position /= creature.chromosome.nodes.len() as f32;

        creature.chromosome.fitness = (creature.starting_position - position).length();
    }

    creatures_created.0 = 0;
}
