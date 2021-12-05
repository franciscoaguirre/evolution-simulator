use super::{
    constants::SINGLE_VALUE_MUTATION_CHANCE,
    operations::{Crossable, Mutable, RandomCreatable},
};
use rand::Rng;

/// Represents the characteristics of a Muscle
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct MusclePhenotype {
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

const MAX_CONTRACTED_TIME: f32 = 1.0;
const MAX_EXTENDED_LENGTH: f32 = 1.0;
const MAX_CONTRACTED_LENGTH: f32 = 1.0;
const MAX_STRENGTH: f32 = 1.0;
const MIN_CONTRACTED_TIME: f32 = 0.0;
const MIN_EXTENDED_LENGTH: f32 = 0.0;
const MIN_CONTRACTED_LENGTH: f32 = 0.0;
const MIN_STRENGTH: f32 = 0.0;

impl Crossable for MusclePhenotype {
    /// Crosses two MusclePhenotypes. Verify that nodes are present in both parents
    fn cross(&self, other: &Self) -> Self {
        MusclePhenotype {
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
        let mut contracted_time = self.contracted_time;
        let mut extended_length = self.extended_length;
        let mut contracted_length = self.contracted_length;
        let mut strength = self.strength;

        if rand::random::<f32>() > SINGLE_VALUE_MUTATION_CHANCE {
            contracted_time = contracted_time + (rand::random::<f32>() - 0.5) * mutation_rate;
            contracted_time = contracted_time
                .max(MIN_CONTRACTED_TIME)
                .min(MAX_CONTRACTED_TIME);
        }
        if rand::random::<f32>() > SINGLE_VALUE_MUTATION_CHANCE {
            extended_length = extended_length + (rand::random::<f32>() - 0.5) * mutation_rate;
            extended_length = extended_length
                .max(MIN_EXTENDED_LENGTH)
                .min(MAX_EXTENDED_LENGTH);
        }
        if rand::random::<f32>() > SINGLE_VALUE_MUTATION_CHANCE {
            contracted_length = contracted_length + (rand::random::<f32>() - 0.5) * mutation_rate;
            contracted_length = contracted_length
                .max(MIN_CONTRACTED_LENGTH)
                .min(MAX_CONTRACTED_LENGTH)
                .min(extended_length);

            extended_length = extended_length.max(contracted_length);
        }
        if rand::random::<f32>() > SINGLE_VALUE_MUTATION_CHANCE {
            strength = strength + (rand::random::<f32>() - 0.5) * mutation_rate;
            strength = strength.max(MIN_STRENGTH).min(MAX_STRENGTH);
        }

        MusclePhenotype {
            contracted_time,
            extended_length,
            contracted_length,
            strength,
            nodes: self.nodes,
        }
    }
}

impl RandomCreatable for MusclePhenotype {
    fn random() -> Self {
        let mut rng = rand::thread_rng();

        MusclePhenotype {
            contracted_time: rand::random::<f32>(),
            extended_length: rand::random::<f32>(),
            contracted_length: rand::random::<f32>(),
            strength: rand::random::<f32>(),
            nodes: (rng.gen_range(0..10), rng.gen_range(0..10)),
        }
    }
}
