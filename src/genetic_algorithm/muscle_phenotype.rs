use super::{
    constants::SINGLE_VALUE_MUTATION_CHANCE,
    operations::{Crossable, Mutable, RandomCreatable},
};
use rand::Rng;

/// Represents the characteristics of a Muscle
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MusclePhenotype {
    /// Percentage of time of the creature's internal clock
    /// the muscle is contracting. The rest of the time
    /// the muscle is extending
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

const MIN_CONTRACTED_TIME: f32 = 0.0;
const MAX_CONTRACTED_TIME: f32 = 1.0;

const MIN_EXTENDED_LENGTH: f32 = 0.08;
const MAX_EXTENDED_LENGTH: f32 = 0.40;

const MIN_CONTRACTED_LENGTH: f32 = 0.08;
const MAX_CONTRACTED_LENGTH: f32 = 0.40;

const MIN_STRENGTH: f32 = 1.0;
const MAX_STRENGTH: f32 = 10.0;

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
            contracted_time += (rand::random::<f32>() - 0.5) * mutation_rate;
            contracted_time = contracted_time
                .max(MIN_CONTRACTED_TIME)
                .min(MAX_CONTRACTED_TIME);
        }
        if rand::random::<f32>() > SINGLE_VALUE_MUTATION_CHANCE {
            extended_length += (rand::random::<f32>() - 0.5) * mutation_rate;
            extended_length = extended_length
                .max(MIN_EXTENDED_LENGTH)
                .min(MAX_EXTENDED_LENGTH);
        }
        if rand::random::<f32>() > SINGLE_VALUE_MUTATION_CHANCE {
            contracted_length += (rand::random::<f32>() - 0.5) * mutation_rate;
            contracted_length = contracted_length
                .max(MIN_CONTRACTED_LENGTH)
                .min(MAX_CONTRACTED_LENGTH)
                .min(extended_length);

            extended_length = extended_length.max(contracted_length);
        }
        if rand::random::<f32>() > SINGLE_VALUE_MUTATION_CHANCE {
            strength += (rand::random::<f32>() - 0.5) * mutation_rate;
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
            extended_length: (rand::random::<f32>() * 2.0)
                .min(MAX_EXTENDED_LENGTH)
                .max(MIN_EXTENDED_LENGTH),
            contracted_length: (rand::random::<f32>() * 2.0)
                .min(MAX_CONTRACTED_LENGTH)
                .max(MIN_CONTRACTED_LENGTH),
            strength: (rand::random::<f32>() + 100.0)
                .min(MAX_STRENGTH)
                .max(MIN_STRENGTH),
            nodes: (rng.gen_range(0..20), rng.gen_range(0..20)),
        }
    }
}
