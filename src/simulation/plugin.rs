use bevy::{core::Stopwatch, prelude::*};
use bevy_rapier3d::prelude::ColliderPosition;

use crate::genetic_algorithm::plugin::{
    FinishedEvaluatingEvent, StartEvaluatingEvent, POPULATION_SIZE,
};

use super::{
    creature::{create_creature, Creature},
    muscle::MusclePlugin,
    node,
};

const EVALUATION_TIME: f32 = 5.0;

struct TimerText;

pub struct SimulationPlugin;

#[derive(Default)]
struct EvaluationStopwatch(Stopwatch);

#[derive(Default)]
struct CreaturesCreated(usize);

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(MusclePlugin)
            .insert_resource(CreaturesCreated::default())
            .insert_resource(Timer::from_seconds(2.0, false))
            .insert_resource(EvaluationStopwatch::default())
            .add_system(simulate.system())
            .add_system(evaluate_simulation.system())
            .add_startup_system(setup_timer_text.system())
            .add_system(update_timer_text.system());
    }
}

fn simulate(
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
        println!("Resetting timer!");
        stopwatch.0.reset();
        stopwatch.0.unpause();
        creatures_created.0 = 0;
    }
}

fn setup_timer_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                ..Default::default()
            },
            text: Text::with_section(
                "0.0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(TimerText);
}

fn update_timer_text(stopwatch: Res<EvaluationStopwatch>, mut query: Query<&mut Text, With<TimerText>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{:.2}", stopwatch.0.elapsed_secs());
    }
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
        let mut position = Vec3::default();
        for (collider_position, parent) in collider_node_positions.iter() {
            if parent.0 != entity {
                continue;
            }

            position += collider_position.0.translation.vector.into();
        }

        position /= creature.chromosome.nodes.len() as f32;

        creature.chromosome.fitness = (creature.starting_position - position).length();
        finished_evaluating_events.send(FinishedEvaluatingEvent {
            chromosome: creature.chromosome.clone(),
        });

        commands
            .entity(entity)
            .despawn_recursive();
    }

    stopwatch.0.pause();
}
