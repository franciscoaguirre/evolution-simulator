use bevy::{prelude::*, tasks::ComputeTaskPool};

use super::{creature, node, plugin::calculate_creatures_position, resources::FitnessStats};

pub struct LoggerPlugin;

impl Plugin for LoggerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(FitnessStats::default())
            .add_system(calculate_creatures_positions.system())
            .add_system(log_fitness.system());
    }
}

fn calculate_creatures_positions(
    pool: Res<ComputeTaskPool>,
    mut creatures: Query<(Entity, &mut creature::Creature)>,
    collider_node_positions: Query<(&Transform, &Parent), With<node::Node>>,
) {
    let span = info_span!("system", name = "log_nodes_fitness");
    let _guard = span.enter();

    creatures.par_for_each_mut(&pool, 32, |(entity, mut creature)| {
        let span_for = info_span!("for", name = "processing creature");
        let _guard = span_for.enter();
        let position = calculate_creatures_position(entity, &collider_node_positions);

        let fitness = position.length();

        if fitness.is_nan() || fitness.is_infinite() {
            return;
        }

        creature.fitness = fitness;
    });
}

fn log_fitness(creatures: Query<&creature::Creature>, mut fitness_stats: ResMut<FitnessStats>) {
    let span = info_span!("system", name = "log_fitness");
    let _guard = span.enter();

    let best = creatures
        .iter()
        .max_by(|x, y| x.fitness.abs().partial_cmp(&y.fitness.abs()).unwrap())
        .unwrap_or(&creature::Creature::default())
        .fitness;
    let worst = creatures
        .iter()
        .min_by(|x, y| x.fitness.abs().partial_cmp(&y.fitness.abs()).unwrap())
        .unwrap_or(&creature::Creature::default())
        .fitness;
    let average = creatures
        .iter()
        .map(|creature| creature.fitness)
        .sum::<f32>()
        / creatures.iter().count() as f32;

    fitness_stats.best = best;
    fitness_stats.worst = worst;
    fitness_stats.average = if average.is_nan() { 0.0 } else { average };
}
