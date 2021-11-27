use super::operations::{Crossable, Mutable};

/// Represents the characteristics of a Muscle
pub struct MusclePhenotype {
    /// Time the muscle is extending
    pub extended_time: f32,
    /// Time the muscle is contracting
    pub contracted_time: f32,
    /// Maximum length the muscle can reach
    pub extended_length: f32,
    /// Minimum length
    pub contracted_length: f32,
    /// The strength with which it pulls/pushes its nodes together/apart
    pub strength: f32,
    /// The nodes this muscle pulls/pushes
    pub nodes: (usize, usize),
}

impl Crossable for MusclePhenotype {
    /// Crosses two MusclePhenotypes. Verify that nodes are present in both parents
    fn cross(&self, other: &Self) -> Self {
        MusclePhenotype {
            extended_time: if rand::random() {
                self.extended_time
            } else {
                other.extended_time
            },
            contracted_time: if rand::random() {
                self.contracted_time
            } else {
                other.contracted_time
            },
            extended_length: if rand::random() {
                self.extended_length
            } else {
                other.extended_length
            },
            contracted_length: if rand::random() {
                self.contracted_length
            } else {
                other.contracted_length
            },
            strength: if rand::random() {
                self.strength
            } else {
                other.strength
            },
            nodes: if rand::random() {
                self.nodes
            } else {
                other.nodes
            },
        }
    }
}

impl Mutable for MusclePhenotype {
    /// Mutates a MusclePhenotype
    fn mutate(&self, mutation_rate: f32) -> Self {
        MusclePhenotype {
            extended_time: self.extended_time + (rand::random::<f32>() - 0.5) * mutation_rate,
            contracted_time: self.contracted_time + (rand::random::<f32>() - 0.5) * mutation_rate,
            extended_length: self.extended_length + (rand::random::<f32>() - 0.5) * mutation_rate,
            contracted_length: self.contracted_length
                + (rand::random::<f32>() - 0.5) * mutation_rate,
            strength: self.strength + (rand::random::<f32>() - 0.5) * mutation_rate,
            nodes: self.nodes,
        }
    }
}
