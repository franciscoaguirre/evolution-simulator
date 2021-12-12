use crate::genetic_algorithm::creature_chromosome::CreatureChromosome;

pub struct StartEvaluatingEvent {
    pub chromosomes: Vec<CreatureChromosome>,
}

pub struct FinishedEvaluatingEvent {
    pub chromosome: CreatureChromosome,
}
