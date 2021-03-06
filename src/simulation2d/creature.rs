use bevy::core::Stopwatch;
use bevy::prelude::*;

use crate::genetic_algorithm::creature_chromosome::CreatureChromosome;

use super::muscle;
use super::node;

#[derive(Default)]
pub struct Creature {
    pub fitness: f32,
    pub chromosome: CreatureChromosome,
    pub internal_clock: Stopwatch,
}

pub fn create_creature(
    commands: &mut Commands,
    creature_chromosome: CreatureChromosome,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    node_size: f32,
) {
    let mut nodes: Vec<Entity> = vec![];

    commands
        .spawn()
        .insert(Creature {
            chromosome: creature_chromosome.clone(),
            internal_clock: Stopwatch::new(),
            ..Default::default()
        })
        .with_children(|parent| {
            for node in creature_chromosome.nodes.iter() {
                let entity =
                    node::create_node(parent, node, meshes, materials, asset_server, node_size);
                nodes.push(entity)
            }

            for muscle in creature_chromosome.muscles.iter() {
                muscle::create_muscle(parent, muscle, &nodes);
            }
        });
}

pub fn create_creature_headless(
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
            ..Default::default()
        })
        .with_children(|parent| {
            for node in creature_chromosome.nodes.iter() {
                let entity = node::create_node_headless(parent, node, node_size);
                nodes.push(entity)
            }

            for muscle in creature_chromosome.muscles.iter() {
                muscle::create_muscle(parent, muscle, &nodes);
            }
        });
}
