use crate::config::CONFIG;

use super::operations::{Crossable, Mutable, RandomCreatable};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents the characteristics of a Node
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq)]
pub struct NodePhenotype {
    /// Initial node position in creature
    pub position: Vec2,
    /// Resistance of node to movement
    pub friction: f32,
}

impl Crossable for NodePhenotype {
    fn cross(&self, other: &Self, chance: f32) -> Self {
        NodePhenotype {
            position: self.position.lerp(other.position, 0.5),
            friction: if rand::random::<f32>() > chance {
                self.friction
            } else {
                other.friction
            },
        }
    }
}

impl Mutable for NodePhenotype {
    fn mutate(&self, chance: f32) -> Self {
        let mut position = self.position;
        let mut friction = self.friction;

        if rand::random::<f32>() > chance * CONFIG.position_mutation_chance_modifier {
            position.x += (rand::random::<f32>() * 2.0 - 1.0) * 0.1;
            position.y += (rand::random::<f32>() * 2.0 - 1.0) * 0.1;
        }

        position.y = position.y.max(0.04);

        if rand::random::<f32>() > chance * CONFIG.single_value_mutation_chance_modifier {
            friction += (rand::random::<f32>() * 2.0 - 1.0) * 0.1;

            friction = friction.clamp(CONFIG.min_friction, CONFIG.max_friction);
        }

        NodePhenotype { position, friction }
    }
}

impl RandomCreatable for NodePhenotype {
    fn random() -> Self {
        NodePhenotype {
            position: Vec2::new(
                (rand::random::<f32>() * 2.0 - 1.0) * 0.4,
                rand::random::<f32>() + 0.04 * 0.4,
            ),
            friction: rand::random::<f32>().clamp(CONFIG.min_friction, CONFIG.max_friction),
        }
    }
}
