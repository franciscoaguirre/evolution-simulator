use std::fs::File;

use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RapierConfiguration,
    prelude::{ColliderPosition, IntegrationParameters},
};
use ron::de::from_reader;

use crate::genetic_algorithm::plugin::{FinishedEvaluatingEvent, StartEvaluatingEvent};

use super::{
    creature::{create_creature, Creature},
    logger::LoggerPlugin,
    muscle::MusclePlugin,
    node,
    resources::{Config, EvaluationStopwatch, GenerationCount},
    ui::UIPlugin,
};

pub struct SimulationPlugin;

#[derive(Default)]
struct CreaturesCreated(usize);

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(UIPlugin)
            .add_plugin(MusclePlugin)
            .add_plugin(LoggerPlugin)
            .insert_resource(match load_config_from_file() {
                Ok(x) => {
                    info!("Loaded config from file");
                    x
                }
                Err(_) => {
                    info!("Config file not found or badly formatted, using default config");
                    Config::default()
                }
            })
            .insert_resource(CreaturesCreated::default())
            .insert_resource(EvaluationStopwatch::default())
            .insert_resource(GenerationCount::default())
            .add_startup_system(apply_config.system())
            .add_system(simulate.system())
            .add_system(evaluate_simulation.system());
    }
}

fn load_config_from_file() -> Result<Config, ron::error::Error> {
    let input_path = "config.ron";
    let file = File::open(&input_path)?;
    let config: Config = from_reader(file)?;
    Ok(config)
}

fn apply_config(
    config: Res<Config>,
    mut integration_parameters: ResMut<IntegrationParameters>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    let inv_dt = integration_parameters.inv_dt();
    integration_parameters.set_inv_dt(inv_dt / config.time_scale);
    rapier_configuration.gravity = Vec2::new(0.0, config.gravity).into();
}

fn simulate(
    mut generation_count: ResMut<GenerationCount>,
    mut creatures_created: ResMut<CreaturesCreated>,
    mut commands: Commands,
    mut stopwatch: ResMut<EvaluationStopwatch>,
    mut start_evaluating_event: EventReader<StartEvaluatingEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<Config>,
) {
    for event in start_evaluating_event.iter() {
        create_creature(
            &mut commands,
            event.chromosome.clone(),
            &mut meshes,
            &mut materials,
            config.node_size,
        );

        creatures_created.0 += 1;
    }

    if creatures_created.0 == config.population_size * 2 {
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
) -> Vec2 {
    let creature_node_count = collider_node_positions
        .iter()
        .filter(|(_, parent)| parent.0 == entity)
        .count();
    let positions_sum: Vec2 = collider_node_positions
        .iter()
        .filter(|(_, parent)| parent.0 == entity)
        .fold(Vec2::ZERO, |sum, (collider_position, _)| {
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
    config: Res<Config>,
) {
    stopwatch.0.tick(time.delta() * config.time_scale as u32);

    if stopwatch.0.paused() || stopwatch.0.elapsed_secs() <= config.evaluation_time {
        return;
    }

    for (entity, mut creature) in creatures.iter_mut() {
        let position = calculate_creatures_position(entity, &collider_node_positions);
        creature.chromosome.fitness = position.length();
        finished_evaluating_events.send(FinishedEvaluatingEvent {
            chromosome: creature.chromosome.clone(),
        });

        commands.entity(entity).despawn_recursive();
    }

    stopwatch.0.pause();
}
