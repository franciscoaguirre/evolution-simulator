use super::crossable::Crossable;
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
