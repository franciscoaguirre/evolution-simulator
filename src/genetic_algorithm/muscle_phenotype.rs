use super::{
    constants::SINGLE_VALUE_MUTATION_CHANCE,
    operations::{Crossable, Mutable},
};

/// Represents the characteristics of a Muscle
#[derive(Debug, Default, Clone, Copy, PartialEq)]
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

const MAX_EXTENDED_TIME: f32 = 1.0;
const MAX_CONTRACTED_TIME: f32 = 1.0;
const MAX_EXTENDED_LENGTH: f32 = 1.0;
const MAX_CONTRACTED_LENGTH: f32 = 1.0;
const MAX_STRENGTH: f32 = 1.0;
const MIN_EXTENDED_TIME: f32 = 0.0;
const MIN_CONTRACTED_TIME: f32 = 0.0;
const MIN_EXTENDED_LENGTH: f32 = 0.0;
const MIN_CONTRACTED_LENGTH: f32 = 0.0;
const MIN_STRENGTH: f32 = 0.0;

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
        let mut extended_time = self.extended_time;
        let mut contracted_time = self.contracted_time;
        let mut extended_length = self.extended_length;
        let mut contracted_length = self.contracted_length;
        let mut strength = self.strength;

        if rand::random::<f32>() > SINGLE_VALUE_MUTATION_CHANCE {
            extended_time = extended_time + (rand::random::<f32>() - 0.5) * mutation_rate;
            extended_time = extended_time.min(MIN_EXTENDED_TIME).max(MAX_EXTENDED_TIME);
        }
        if rand::random::<f32>() > SINGLE_VALUE_MUTATION_CHANCE {
            contracted_time = contracted_time + (rand::random::<f32>() - 0.5) * mutation_rate;
            contracted_time = contracted_time
                .min(MIN_CONTRACTED_TIME)
                .max(MAX_CONTRACTED_TIME);
        }
        if rand::random::<f32>() > SINGLE_VALUE_MUTATION_CHANCE {
            extended_length = extended_length + (rand::random::<f32>() - 0.5) * mutation_rate;
            extended_length = extended_length
                .min(MIN_EXTENDED_LENGTH)
                .max(MAX_EXTENDED_LENGTH);
        }
        if rand::random::<f32>() > SINGLE_VALUE_MUTATION_CHANCE {
            contracted_length = contracted_length + (rand::random::<f32>() - 0.5) * mutation_rate;
            contracted_length = contracted_length
                .min(MIN_CONTRACTED_LENGTH)
                .max(MAX_CONTRACTED_LENGTH)
                .max(extended_length);

            extended_length = extended_length.min(contracted_length);
        }
        if rand::random::<f32>() > SINGLE_VALUE_MUTATION_CHANCE {
            strength = strength + (rand::random::<f32>() - 0.5) * mutation_rate;
            strength = strength.min(MIN_STRENGTH).max(MAX_STRENGTH);
        }

        MusclePhenotype {
            extended_time,
            contracted_time,
            extended_length,
            contracted_length,
            strength,
            nodes: self.nodes,
        }
    }
}
