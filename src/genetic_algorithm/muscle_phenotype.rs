use crate::config::CONFIG;

use super::operations::{Crossable, Mutable, RandomCreatable};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Represents the characteristics of a Muscle
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
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
    fn mutate(&self, chance: f32) -> Self {
        let mut contracted_time = self.contracted_time;
        let mut extended_length = self.extended_length;
        let mut contracted_length = self.contracted_length;
        let mut strength = self.strength;

        if rand::random::<f32>() > chance * CONFIG.single_value_mutation_chance_modifier {
            contracted_time += rand::random::<f32>() - 0.5;
            contracted_time =
                contracted_time.clamp(CONFIG.min_contracted_time, CONFIG.max_contracted_time);
        }
        if rand::random::<f32>() > chance * CONFIG.single_value_mutation_chance_modifier {
            extended_length += rand::random::<f32>() - 0.5;
            extended_length =
                extended_length.clamp(CONFIG.min_extended_length, CONFIG.max_extended_length);
        }
        if rand::random::<f32>() > chance * CONFIG.single_value_mutation_chance_modifier {
            contracted_length += rand::random::<f32>() - 0.5;
            contracted_length = contracted_length
                .clamp(CONFIG.min_contracted_length, CONFIG.max_contracted_length)
                .min(extended_length);

            extended_length = extended_length.max(contracted_length);
        }
        if rand::random::<f32>() > chance * CONFIG.single_value_mutation_chance_modifier {
            strength += rand::random::<f32>() - 0.5;
            strength = strength.clamp(CONFIG.min_strength, CONFIG.max_strength);
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
                .clamp(CONFIG.min_extended_length, CONFIG.max_extended_length),
            contracted_length: (rand::random::<f32>() * 2.0)
                .clamp(CONFIG.min_extended_length, CONFIG.max_extended_length),
            strength: (rand::random::<f32>() + 100.0)
                .clamp(CONFIG.min_strength, CONFIG.max_strength),
            nodes: (rng.gen_range(0..20), rng.gen_range(0..20)),
        }
    }
}
