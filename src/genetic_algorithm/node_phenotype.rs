use super::{
    constants::{POSITION_MUTATION_CHANCE, SINGLE_VALUE_MUTATION_CHANCE},
    operations::{Crossable, Mutable, RandomCreatable},
};
use bevy::prelude::*;

/// Represents the characteristics of a Node
#[derive(Debug, Default, Clone, Copy, PartialEq)]
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
        let mut friction = self.friction;

        if rand::random::<f32>() > POSITION_MUTATION_CHANCE {
            position.x += (rand::random::<f32>() - 0.5) * mutation_rate;
            position.y += (rand::random::<f32>() - 0.5) * mutation_rate;
            position.z += (rand::random::<f32>() - 0.5) * mutation_rate;
        }

        if rand::random::<f32>() > SINGLE_VALUE_MUTATION_CHANCE {
            friction += (rand::random::<f32>() + 0.1) * mutation_rate;
        }

        NodePhenotype { position, friction }
    }
}

impl RandomCreatable for NodePhenotype {
    fn random() -> Self {
        NodePhenotype {
            position: Vec3::new(
                (rand::random::<f32>() * 2.0 - 1.0) * 10.0,
                rand::random::<f32>() * 2.0,
                (rand::random::<f32>() * 2.0 - 1.0) * 10.0,
            ),
            friction: rand::random::<f32>() * 2.0,
        }
    }
}
