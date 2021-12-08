use std::fs::File;

use bevy::prelude::*;

use ron::de::from_reader;

use crate::genetic_algorithm::plugin::{FinishedEvaluatingEvent, StartEvaluatingEvent};

use super::{
    creature::{create_creature, Creature},
    logger::LoggerPlugin,
    muscle::MusclePlugin,
    node,
    physics::PhysicsPlugin,
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
            .add_plugin(PhysicsPlugin)
            .insert_resource(match load_config_from_file() {
                Ok(x) => {
                    info!("Loaded config from file");
                    x
                }
                Err(err) => {
                    warn!(
                        "Config file error. {}. Using default config.",
                        err.code.to_string()
                    );
                    Config::default()
                }
            })
            .insert_resource(CreaturesCreated::default())
            .insert_resource(EvaluationStopwatch::default())
            .insert_resource(GenerationCount::default())
            .add_system(simulate.system())
            .add_system(evaluate_simulation.system())
            .add_system(restart_stopwatch.system())
            .add_system(tick_stopwatch.system());
    }
}

fn load_config_from_file() -> Result<Config, ron::error::Error> {
    let input_path = "config.ron";
    let file = File::open(&input_path)?;
    let config: Config = from_reader(file)?;
    Ok(config)
}

fn simulate(
    mut creatures_created: ResMut<CreaturesCreated>,
    mut commands: Commands,
    mut start_evaluating_events: EventReader<StartEvaluatingEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    for event in start_evaluating_events.iter() {
        create_creature(
            &mut commands,
            event.chromosome.clone(),
            &mut meshes,
            &mut materials,
            &asset_server,
            config.node_size,
        );

        creatures_created.0 += 1;

        dbg!(creatures_created.0);
    }
}

fn restart_stopwatch(
    mut generation_count: ResMut<GenerationCount>,
    config: Res<Config>,
    mut creatures_created: ResMut<CreaturesCreated>,
    mut stopwatch: ResMut<EvaluationStopwatch>,
) {
    if creatures_created.0 == config.population_size * 2 {
        stopwatch.0.reset();
        stopwatch.0.unpause();
        generation_count.0 += 1;

        creatures_created.0 = 0;
    }
}

fn tick_stopwatch(
    time: Res<Time>,
    mut stopwatch: ResMut<EvaluationStopwatch>,
    config: Res<Config>,
) {
    stopwatch.0.tick(time.delta() * config.time_scale as u32);
}

/// Calculates creature's position averaging its nodes positions
pub fn calculate_creatures_position(
    entity: Entity,
    collider_node_positions: &Query<(&Transform, &Parent), With<node::Node>>,
) -> Vec3 {
    let creature_node_count = collider_node_positions
        .iter()
        .filter(|(_, parent)| parent.0 == entity)
        .count();
    let positions_sum: Vec3 = collider_node_positions
        .iter()
        .filter(|(_, parent)| parent.0 == entity)
        .fold(Vec3::ZERO, |sum, (collider_position, _)| {
            sum + collider_position.translation
        });
    positions_sum / creature_node_count as f32
}

fn evaluate_simulation(
    mut commands: Commands,
    stopwatch: ResMut<EvaluationStopwatch>,
    mut creatures: Query<(Entity, &mut Creature)>,
    collider_node_positions: Query<(&Transform, &Parent), With<node::Node>>,
    mut finished_evaluating_events: EventWriter<FinishedEvaluatingEvent>,
    config: Res<Config>,
) {
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
}
