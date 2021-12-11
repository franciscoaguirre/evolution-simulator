use std::time::Duration;

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use structopt::StructOpt;

use super::constants::{FIXED_TIME_STEP, FIXED_TIME_STEP_NANOSECONDS, TIME_SCALE};
use super::creature::Creature;
use super::node;
use super::physics::Velocity;
use super::resources::Config;
use crate::arguments::Opt;
use crate::genetic_algorithm::muscle_phenotype::MusclePhenotype;

pub struct Muscle {
    contracted_time: f32,
    contracted_length: f32,
    extended_length: f32,
    strength: f32,
    nodes: (Entity, Entity),
}

#[derive(Bundle)]
pub struct MuscleBundle {
    muscle: Muscle,
}

impl Muscle {
    fn from_phenotype(muscle_phenotype: &MusclePhenotype, nodes: &[Entity]) -> Muscle {
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
    nodes: &[Entity],
) {
    parent
        .spawn()
        .insert(Muscle::from_phenotype(muscle_phenotype, nodes));
}

pub struct MusclePlugin;

impl Plugin for MusclePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let options = Opt::from_args();

        if !options.headless {
            app.add_plugin(DebugLinesPlugin)
                .add_system(draw_muscles.system());
        }

        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64 / TIME_SCALE))
                .with_system(advance_internal_clocks.system())
                .with_system(apply_forces.system()),
        );
    }
}

fn advance_internal_clocks(mut creatures: Query<&mut Creature>) {
    let span = info_span!("system", name = "advance_internal_clocks");
    let _guard = span.enter();

    for mut creature in creatures.iter_mut() {
        creature
            .internal_clock
            .tick(Duration::from_nanos(FIXED_TIME_STEP_NANOSECONDS));

        if creature.internal_clock.elapsed_secs() >= creature.chromosome.internal_clock_size {
            creature.internal_clock.reset();
        }
    }
}

fn draw_muscles(
    muscles: Query<&Muscle>,
    nodes: Query<&Transform, With<node::Node>>,
    mut lines: ResMut<DebugLines>,
) {
    let span = info_span!("system", name = "draw_muscles");
    let _guard = span.enter();

    for muscle in muscles.iter() {
        let start = nodes.get(muscle.nodes.0).unwrap().translation;
        let end = nodes.get(muscle.nodes.1).unwrap().translation;

        lines.line(start, end, 0.0);
    }
}

fn apply_forces(
    muscles: Query<(&Muscle, &Parent)>,
    node_positions: Query<&Transform, With<node::Node>>,
    mut node_velocities: Query<&mut Velocity, With<node::Node>>,
    creatures: Query<&Creature>,
    config: Res<Config>,
) {
    let span = info_span!("system", name = "apply_forces");
    let _guard = span.enter();

    let delta_time = FIXED_TIME_STEP;

    for (muscle, parent) in muscles.iter() {
        let creature = creatures.get(parent.0).unwrap();
        let internal_clock_size = creature.chromosome.internal_clock_size;
        let should_contract =
            creature.internal_clock.elapsed_secs() <= muscle.contracted_time * internal_clock_size;
        let target_length = if should_contract {
            muscle.contracted_length
        } else {
            muscle.extended_length
        };

        let first_node_position = node_positions.get(muscle.nodes.0).unwrap().translation;
        let second_node_position = node_positions.get(muscle.nodes.1).unwrap().translation;
        let mut first_to_second_direction =
            (second_node_position - first_node_position).normalize();

        if first_node_position == second_node_position {
            first_to_second_direction = Vec3::new(0.0, 0.1, 0.0);
        }

        let second_to_first_direction = -first_to_second_direction;
        let muscle_length = (second_node_position - first_node_position).length();

        let sign = (target_length - muscle_length).signum();
        let force =
            ((muscle_length - target_length) / muscle_length.max(target_length)).powf(2.0) * sign;

        let strength = muscle.strength * (1.0 / config.air_friction);

        let mut first_node_velocity = node_velocities.get_mut(muscle.nodes.0).unwrap();
        first_node_velocity.0 += second_to_first_direction * force * strength * delta_time;

        let mut second_node_velocity = node_velocities.get_mut(muscle.nodes.1).unwrap();

        second_node_velocity.0 += first_to_second_direction * force * strength * delta_time;
    }
}
