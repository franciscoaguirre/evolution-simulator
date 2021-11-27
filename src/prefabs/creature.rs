use bevy::prelude::*;

use crate::prefabs::node;
use crate::prefabs::muscle;

#[derive(Default)]
pub struct Creature {
    pub position: Vec3,
    pub nodes: Vec<node::Node>,
    pub muscles: Vec<muscle::Muscle>,
}

pub fn create_creature(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, creature: Creature) {
    commands.spawn()
        .with_children(|parent| {

            for node in creature.nodes.iter() {
                node::create_node(parent, meshes, materials, &node, creature.position);
            }

            for muscle in creature.muscles.iter() {
                muscle::create_muscle(parent, &muscle);
            }

        });
}
