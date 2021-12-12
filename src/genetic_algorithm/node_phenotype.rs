use crate::config::CONFIG;

use super::operations::{Correctable, Crossable, Mutable, RandomCreatable};
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

impl Correctable for NodePhenotype {
    fn correct(&mut self) {
        self.position = Vec2::new(
            self.position
                .x
                .clamp(-CONFIG.max_extended_length, CONFIG.max_extended_length),
            self.position.y.clamp(0.04, CONFIG.max_extended_length),
        );
        self.friction = self
            .friction
            .clamp(CONFIG.min_friction, CONFIG.max_friction);
    }

    fn is_correct(&self) -> bool {
        (-CONFIG.max_extended_length..CONFIG.max_extended_length).contains(&self.position.x)
            && (0.04..CONFIG.max_extended_length).contains(&self.position.y)
            && (CONFIG.min_friction..CONFIG.max_friction).contains(&self.friction)
    }
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
                (rand::random::<f32>() * 2.0 - 1.0) * CONFIG.max_extended_length,
                rand::random::<f32>() + 0.04 * CONFIG.max_extended_length,
            ),
            friction: rand::random::<f32>().clamp(CONFIG.min_friction, CONFIG.max_friction),
        }
    }
}
