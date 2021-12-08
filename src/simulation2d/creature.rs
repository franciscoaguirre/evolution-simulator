use bevy::core::Stopwatch;
use bevy::prelude::*;

use crate::genetic_algorithm::creature_chromosome::CreatureChromosome;

use super::muscle;
use super::node;

pub struct Creature {
    pub chromosome: CreatureChromosome,
    pub internal_clock: Stopwatch,
}

// #[derive(Bundle)]
// struct CreatureBundle {
//     creature: Creature,

//     // We can nest/include another bundle.
//     // Add the components for a standard Bevy Sprite:
//     #[bundle]
//     nodes: Children,
//     #[bundle]
//     muscles: Vec::<muscle::MuscleBundle>,
// }


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

// pub fn create_creature_bundle(
//     commands: &mut Commands,
//     creature_chromosome: CreatureChromosome,
//     meshes: &mut ResMut<Assets<Mesh>>,
//     materials: &mut ResMut<Assets<ColorMaterial>>,
//     asset_server: &Res<AssetServer>,
//     node_size: f32,
// ) -> CreatureBundle {
//     let mut nodes: Vec<Entity> = vec![];

//     commands
//         .spawn()
//         .insert(Creature {
//             chromosome: creature_chromosome.clone(),
//             internal_clock: Stopwatch::new(),
//         })
//         .with_children(|parent| {
//             for node in creature_chromosome.nodes.iter() {
//                 let entity =
//                     node::create_node(parent, node, meshes, materials, asset_server, node_size);
//                 nodes.push(entity)
//             }

//             for muscle in creature_chromosome.muscles.iter() {
//                 muscle::create_muscle(parent, muscle, &nodes);
//             }
//         });
// }
