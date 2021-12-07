use bevy::core::Stopwatch;
use bevy::prelude::*;

use crate::genetic_algorithm::creature_chromosome::CreatureChromosome;

use super::muscle;
use super::node;

pub struct Creature {
    pub chromosome: CreatureChromosome,
    pub internal_clock: Stopwatch,
}

pub fn create_creature(
    commands: &mut Commands,
    creature_chromosome: CreatureChromosome,
    node_size: f32,
) {
    let mut nodes: Vec<Entity> = vec![];

    commands
        .spawn()
        .insert(Creature {
            chromosome: creature_chromosome.clone(),
            internal_clock: Stopwatch::new(),
        })
        .with_children(|parent| {
            for node in creature_chromosome.nodes.iter() {
                let entity =
                    node::create_node(parent,  node, node_size);
                nodes.push(entity)
            }

            for muscle in creature_chromosome.muscles.iter() {
                muscle::create_muscle(parent, muscle, &nodes);
            }
        });
}
