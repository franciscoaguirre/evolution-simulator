use bevy::prelude::*;
use bevy_rapier3d::prelude::ColliderPosition;

use crate::genetic_algorithm::plugin::{
    FinishedEvaluatingEvent, StartEvaluatingEvent, POPULATION_SIZE,
};

use super::{
    creature::{create_creature, Creature},
    logger::LoggerPlugin,
    muscle::MusclePlugin,
    node,
    resources::{EvaluationStopwatch, GenerationCount},
    ui::UIPlugin,
};

const EVALUATION_TIME: f32 = 5.0;

pub struct SimulationPlugin;

#[derive(Default)]
struct CreaturesCreated(usize);

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(UIPlugin)
            .add_plugin(MusclePlugin)
            .add_plugin(LoggerPlugin)
            .insert_resource(CreaturesCreated::default())
            .insert_resource(EvaluationStopwatch::default())
            .insert_resource(GenerationCount::default())
            .add_system(simulate.system())
            .add_system(evaluate_simulation.system());
    }
}

fn simulate(
    mut generation_count: ResMut<GenerationCount>,
    mut creatures_created: ResMut<CreaturesCreated>,
    mut commands: Commands,
    mut stopwatch: ResMut<EvaluationStopwatch>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut start_evaluating_event: EventReader<StartEvaluatingEvent>,
) {
    for event in start_evaluating_event.iter() {
        create_creature(
            &mut commands,
            &mut meshes,
            &mut materials,
            event.chromosome.clone(),
            Vec3::default() + Vec3::new(10.0, 0.0, 0.0) * creatures_created.0 as f32,
        );

        creatures_created.0 += 1;
    }

    if creatures_created.0 == POPULATION_SIZE * 2 {
        stopwatch.0.reset();
        stopwatch.0.unpause();
        generation_count.0 += 1;
        creatures_created.0 = 0;
    }
}

/// Calculates creature's position averaging its nodes positions
pub fn calculate_creatures_position(
    entity: Entity,
    collider_node_positions: &Query<(&ColliderPosition, &Parent), With<node::Node>>,
) -> Vec3 {
    let creature_node_count = collider_node_positions
        .iter()
        .filter(|(_, parent)| parent.0 == entity)
        .count();
    let positions_sum: Vec3 = collider_node_positions
        .iter()
        .filter(|(_, parent)| parent.0 == entity)
        .fold(Vec3::ZERO, |sum, (collider_position, _)| {
            sum + collider_position.0.translation.vector.into()
        });
    positions_sum / creature_node_count as f32
}

fn evaluate_simulation(
    mut commands: Commands,
    time: Res<Time>,
    mut stopwatch: ResMut<EvaluationStopwatch>,
    mut creatures: Query<(Entity, &mut Creature)>,
    collider_node_positions: Query<(&ColliderPosition, &Parent), With<node::Node>>,
    mut finished_evaluating_events: EventWriter<FinishedEvaluatingEvent>,
) {
    stopwatch.0.tick(time.delta());

    if stopwatch.0.paused() || stopwatch.0.elapsed_secs() <= EVALUATION_TIME {
        return;
    }

    for (entity, mut creature) in creatures.iter_mut() {
        let position = calculate_creatures_position(entity, &collider_node_positions);
        creature.chromosome.fitness = (creature.starting_position - position).length();
        finished_evaluating_events.send(FinishedEvaluatingEvent {
            chromosome: creature.chromosome.clone(),
        });

        commands.entity(entity).despawn_recursive();
    }

    stopwatch.0.pause();
}
