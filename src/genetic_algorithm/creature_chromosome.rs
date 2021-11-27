use crate::genetic_algorithm::*;

#[derive(Default)]
pub struct CreatureChromosome {
    pub nodes: Vec<node_phenotype::NodePhenotype>,
    pub muscles: Vec<muscle_phenotype::MusclePhenotype>,
}
