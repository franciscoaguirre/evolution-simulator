use std::{fs::File, time::Duration};

use bevy::{core::FixedTimestep, prelude::*};

use ron::de::from_reader;

use crate::genetic_algorithm::plugin::{
    CreatureSpeciesGA, FinishedEvaluatingEvent, StartEvaluatingEvent,
};

use super::{
    constants::{FIXED_TIME_STEP, FIXED_TIME_STEP_NANOSECONDS, TIME_SCALE},
    creature::{create_creature, Creature},
    muscle::MusclePlugin,
    node,
    physics::PhysicsPlugin,
    resources::{Config, EvaluationStopwatch, GenerationCount},
    ui::UIPlugin,
};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(UIPlugin)
            .add_plugin(MusclePlugin)
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
            .insert_resource(EvaluationStopwatch::default())
            .insert_resource(GenerationCount::default())
            .add_system(simulate.system())
            .add_system(evaluate_simulation.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64 / TIME_SCALE))
                    .with_system(tick_stopwatch.system()),
            );
    }
}

fn load_config_from_file() -> Result<Config, ron::error::Error> {
    let input_path = "config.ron";
    let file = File::open(&input_path)?;
    let config: Config = from_reader(file)?;
    Ok(config)
}

fn simulate(
    ga: Res<CreatureSpeciesGA>,
    mut commands: Commands,
    mut start_evaluating_events: EventReader<StartEvaluatingEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut stopwatch: ResMut<EvaluationStopwatch>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    let span = info_span!("system", name = "simulate");
    let _guard = span.enter();

    for _event in start_evaluating_events.iter() {
        for chromosome in ga
            .population
            .values()
            .flatten()
            .chain(ga.offspring_population.iter())
        {
            create_creature(
                &mut commands,
                chromosome.clone(),
                &mut meshes,
                &mut materials,
                &asset_server,
                config.node_size,
            );
        }

        stopwatch.0.reset();
        stopwatch.0.unpause();
    }
}

fn tick_stopwatch(mut stopwatch: ResMut<EvaluationStopwatch>) {
    let span = info_span!("system", name = "tick_stopwatch");
    let _guard = span.enter();

    stopwatch
        .0
        .tick(Duration::from_nanos(FIXED_TIME_STEP_NANOSECONDS));
}

/// Calculates creature's position averaging its nodes positions
pub fn calculate_creatures_position(
    entity: Entity,
    collider_node_positions: &Query<(&Transform, &Parent), With<node::Node>>,
) -> Vec3 {
    let span = info_span!("helper", name = "calculate_creatures_position");
    let _guard = span.enter();

    let (creature_node_count, positions_sum) = collider_node_positions
        .iter()
        .filter(|(_, parent)| parent.0 == entity)
        .fold((0, Vec3::ZERO), |(count, sum), (collider_position, _)| {
            (count + 1, sum + collider_position.translation)
        });
    positions_sum / creature_node_count as f32
}

fn evaluate_simulation(
    mut commands: Commands,
    mut stopwatch: ResMut<EvaluationStopwatch>,
    mut creatures: Query<(Entity, &mut Creature)>,
    collider_node_positions: Query<(&Transform, &Parent), With<node::Node>>,
    mut finished_evaluating_events: EventWriter<FinishedEvaluatingEvent>,
    config: Res<Config>,
) {
    let span = info_span!("system", name = "evaluate_simulation");
    let _guard = span.enter();

    if stopwatch.0.paused() || stopwatch.0.elapsed_secs() <= config.evaluation_time {
        return;
    }

    for (entity, mut creature) in creatures.iter_mut() {
        let position = calculate_creatures_position(entity, &collider_node_positions);
        creature.chromosome.fitness = position.x.abs();
        finished_evaluating_events.send(FinishedEvaluatingEvent {
            chromosome: creature.chromosome.clone(),
        });

        commands.entity(entity).despawn_recursive();
    }

    stopwatch.0.pause();
}
