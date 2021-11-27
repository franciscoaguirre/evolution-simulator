use crate::genetic_algorithm::*;

use super::{breedable::Breedable, crossable::Crossable};

#[derive(Default)]
pub struct CreatureChromosome {
    pub nodes: Vec<node_phenotype::NodePhenotype>,
    pub muscles: Vec<muscle_phenotype::MusclePhenotype>,
}

impl Crossable for CreatureChromosome {
    fn cross(&self, other: &Self) -> Self {
        CreatureChromosome {
            nodes: self
                .nodes
                .iter()
                .zip(other.nodes.iter())
                .map(|(a, b)| a.cross(b))
                .collect(),
            muscles: self
                .muscles
                .iter()
                .zip(other.muscles.iter())
                .map(|(a, b)| a.cross(b))
                .collect(),
        }
    }
}

impl Breedable for CreatureChromosome {
    fn breed(&self, other: &Self) -> (Self, Self)
    where
        Self: Sized,
    {
        (self.cross(other), self.cross(other))
    }
}
