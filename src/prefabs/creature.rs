use bevy::prelude::*;

mod node;
use node::*;

struct Creature {
    pub nodes: Vec<Node>,
}

pub fn create_creature(commands: &mut Commands) {}
