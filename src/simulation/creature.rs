use bevy::core::Stopwatch;
use bevy::prelude::*;

use crate::genetic_algorithm::creature_chromosome::CreatureChromosome;

use super::muscle;
use super::node;

pub struct Creature {
    pub chromosome: CreatureChromosome,
    pub starting_position: Vec3,
    pub internal_clock: Stopwatch,
}

pub fn create_creature(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    creature_chromosome: CreatureChromosome,
    position: Vec3,
) {
    let mut nodes: Vec<Entity> = vec![];

    commands
        .spawn()
        .insert(Creature {
            chromosome: creature_chromosome.clone(),
            starting_position: position,
            internal_clock: Stopwatch::new(),
        })
        .with_children(|parent| {
            for node in creature_chromosome.nodes.iter() {
                let entity = node::create_node(parent, meshes, materials, node, position);
                nodes.push(entity)
            }

            for muscle in creature_chromosome.muscles.iter() {
                muscle::create_muscle(parent, muscle, &nodes);
            }
        });
}
