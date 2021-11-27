use super::operations::{Crossable, Mutable};
use bevy::prelude::*;

/// Represents the characteristics of a Node
pub struct NodePhenotype {
    /// Initial node position in creature
    pub position: Vec3,
    /// Resistance of node to movement
    pub friction: f32,
}

impl Crossable for NodePhenotype {
    fn cross(&self, other: &Self) -> Self {
        NodePhenotype {
            position: self.position.lerp(other.position, 0.5),
            friction: if rand::random() {
                self.friction
            } else {
                other.friction
            },
        }
    }
}

impl Mutable for NodePhenotype {
    fn mutate(&self, mutation_rate: f32) -> Self {
        let mut position = self.position.clone();
        position.x += (rand::random::<f32>() - 0.5) * mutation_rate;
        position.y += (rand::random::<f32>() - 0.5) * mutation_rate;
        position.z += (rand::random::<f32>() - 0.5) * mutation_rate;

        NodePhenotype {
            position,
            friction: self.friction + (rand::random::<f32>() - 0.5) * mutation_rate,
        }
    }
}
