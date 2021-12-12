use std::time::Duration;

use bevy::{app::AppExit, core::FixedTimestep, prelude::*};

use structopt::StructOpt;

use crate::{
    arguments::Opt,
    config::CONFIG,
    genetic_algorithm::plugin::{FinishedEvaluatingEvent, GeneticAlgorithm, StartEvaluatingEvent},
};

use super::{
    creature::{create_creature, create_creature_headless, Creature},
    muscle::MusclePlugin,
    node,
    physics::PhysicsPlugin,
    resources::{EvaluationStopwatch, GenerationCount, RealTimeStopwatch},
};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(MusclePlugin)
            .add_plugin(PhysicsPlugin)
            .insert_resource(EvaluationStopwatch::default())
            .insert_resource(GenerationCount::default())
            .insert_resource(RealTimeStopwatch::default())
            .add_system(evaluate_simulation.system())
            .add_system(real_stopwatch_ticker.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(
                        CONFIG.fixed_time_step as f64 / CONFIG.time_scale as f64,
                    ))
                    .with_system(tick_stopwatch.system()),
            )
            .add_system(check_should_end_simulation.system());

        let options = Opt::from_args();

        if options.headless {
            app.add_system(simulate_headless.system());
        } else {
            app.add_system(simulate.system());
        }
    }
}

fn simulate(
    ga: Res<GeneticAlgorithm>,
    mut commands: Commands,
    mut start_evaluating_events: EventReader<StartEvaluatingEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut stopwatch: ResMut<EvaluationStopwatch>,
    asset_server: Res<AssetServer>,
) {
    let span = info_span!("system", name = "simulate");
    let _guard = span.enter();

    for _event in start_evaluating_events.iter() {
        for chromosome in ga.algorithm.get_population_for_sim() {
            create_creature(
                &mut commands,
                chromosome.clone(),
                &mut meshes,
                &mut materials,
                &asset_server,
                CONFIG.node_size,
            );
        }

        stopwatch.0.reset();
        stopwatch.0.unpause();
    }
}

fn simulate_headless(
    ga: Res<GeneticAlgorithm>,
    mut commands: Commands,
    mut start_evaluating_events: EventReader<StartEvaluatingEvent>,
    mut stopwatch: ResMut<EvaluationStopwatch>,
    mut real_stopwatch: ResMut<RealTimeStopwatch>,
) {
    let span = info_span!("system", name = "simulate");
    let _guard = span.enter();

    for _event in start_evaluating_events.iter() {
        for chromosome in ga.algorithm.get_population_for_sim() {
            create_creature_headless(&mut commands, chromosome.clone(), CONFIG.node_size);
        }

        println!("Time spent: {}", real_stopwatch.0.elapsed().as_millis());
        stopwatch.0.reset();
        stopwatch.0.unpause();

        real_stopwatch.0.reset();
    }
}

fn tick_stopwatch(mut stopwatch: ResMut<EvaluationStopwatch>) {
    let span = info_span!("system", name = "tick_stopwatch");
    let _guard = span.enter();

    stopwatch
        .0
        .tick(Duration::from_nanos((CONFIG.fixed_time_step * 1e9) as u64));
}

fn real_stopwatch_ticker(mut real_stopwatch: ResMut<RealTimeStopwatch>, time: Res<Time>) {
    real_stopwatch.0.tick(time.delta());
}

/// Calculates creature's position averaging its nodes positions
pub fn calculate_creatures_position(
    entity: Entity,
    collider_node_positions: &Query<(&Transform, &Parent), With<node::Node>>,
) -> f32 {
    let span = info_span!("helper", name = "calculate_creatures_position");
    let _guard = span.enter();

    let (creature_node_count, positions_sum) = collider_node_positions
        .iter()
        .filter(|(_, parent)| parent.0 == entity)
        .fold((0, 0.0), |(count, sum), (collider_position, _)| {
            (count + 1, sum + collider_position.translation.x)
        });
    positions_sum / creature_node_count as f32
}

fn evaluate_simulation(
    mut commands: Commands,
    mut stopwatch: ResMut<EvaluationStopwatch>,
    mut creatures: Query<(Entity, &mut Creature)>,
    collider_node_positions: Query<(&Transform, &Parent), With<node::Node>>,
    mut finished_evaluating_events: EventWriter<FinishedEvaluatingEvent>,
) {
    let span = info_span!("system", name = "evaluate_simulation");
    let _guard = span.enter();

    if stopwatch.0.paused() || stopwatch.0.elapsed_secs() <= CONFIG.evaluation_time {
        return;
    }

    for (entity, mut creature) in creatures.iter_mut() {
        let position_x = calculate_creatures_position(entity, &collider_node_positions);
        creature.chromosome.fitness = position_x.abs();
        finished_evaluating_events.send(FinishedEvaluatingEvent {
            chromosome: creature.chromosome.clone(),
        });

        commands.entity(entity).despawn_recursive();
    }

    stopwatch.0.pause();
}

fn check_should_end_simulation(
    ga: Res<GeneticAlgorithm>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if ga.algorithm.get_should_end() {
        app_exit_events.send(AppExit);
    };
}
