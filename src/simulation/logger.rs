use bevy::prelude::*;
use bevy_rapier3d::prelude::ColliderPosition;

use super::{
    creature::Creature, node, plugin::calculate_creatures_position, resources::FitnessStats,
};

pub struct LoggerPlugin;

impl Plugin for LoggerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(FitnessStats::default())
            .add_system(log_nodes_fitness.system());
    }
}

fn log_nodes_fitness(
    mut fitness_stats: ResMut<FitnessStats>,
    creatures: Query<(Entity, &Creature)>,
    collider_node_positions: Query<(&ColliderPosition, &Parent), With<node::Node>>,
) {
    let mut fitnesses: Vec<f32> = Vec::new();

    for (entity, creature) in creatures.iter() {
        let position = calculate_creatures_position(entity, &collider_node_positions);

        let fitness = (creature.starting_position - position).length();

        if fitness.is_nan() || fitness.is_infinite() {
            continue;
        }

        fitnesses.push((creature.starting_position - position).length());
    }

    let best = fitnesses
        .iter()
        .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
        .unwrap_or(&0.0);
    let worst = fitnesses
        .iter()
        .min_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
        .unwrap_or(&0.0);
    let average = fitnesses.iter().sum::<f32>() / fitnesses.len() as f32;

    fitness_stats.best = *best;
    fitness_stats.worst = *worst;
    fitness_stats.average = if average.is_nan() { 0.0 } else { average };
}
