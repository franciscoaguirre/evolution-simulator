use rand::Rng;

use crate::genetic_algorithm::*;

use super::{
    muscle_phenotype::MusclePhenotype,
    node_phenotype::NodePhenotype,
    operations::{Breedable, Crossable, Mutable},
};

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

impl Mutable for CreatureChromosome {
    fn mutate(&self, mutation_rate: f32) -> Self {
        let mut nodes: Vec<NodePhenotype> = self
            .nodes
            .iter()
            .map(|node| node.mutate(mutation_rate))
            .collect();
        let mut muscles: Vec<MusclePhenotype> = self
            .muscles
            .iter()
            .map(|muscle| muscle.mutate(mutation_rate))
            .collect();

        let node_index_remove: usize = rand::thread_rng().gen_range(0, nodes.len());
        let muscle_index_remove: usize = rand::thread_rng().gen_range(0, muscles.len());

        if rand::random() {
            nodes.remove(node_index_remove);
        }
        if rand::random() {
            muscles.remove(muscle_index_remove);
        }

        CreatureChromosome { nodes, muscles }
    }
}
