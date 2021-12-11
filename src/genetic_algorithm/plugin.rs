use bevy::prelude::*;

use crate::simulation2d::resources::{Config, GenerationCount};

use super::{
    algorithms::{
        algorithm::Algorithm, runner::Runnable, species_based_algorithm::SpeciesBasedAlgorithm,
    },
    creature_chromosome::CreatureChromosome,
};

pub struct GeneticAlgorithmPlugin;

pub struct StartEvaluatingEvent;

pub struct FinishedEvaluatingEvent {
    pub chromosome: CreatureChromosome,
}

#[derive(Default)]
struct FinishedEvaluatingCounter(usize);

pub type CreatureGA = Algorithm<CreatureChromosome>;
pub type CreatureSpeciesGA = SpeciesBasedAlgorithm<CreatureChromosome>;

impl Plugin for GeneticAlgorithmPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(CreatureSpeciesGA::default())
            .insert_resource(FinishedEvaluatingCounter::default())
            .add_event::<StartEvaluatingEvent>()
            .add_event::<FinishedEvaluatingEvent>()
            .add_startup_system(setup_genetic_algorithm.system())
            .add_system(count_finished_evaluating.system())
            .add_system(genetic_algorithm_system.system());
    }
}

fn setup_genetic_algorithm(
    mut ga: ResMut<CreatureSpeciesGA>,
    mut start_evaluating_events: EventWriter<StartEvaluatingEvent>,
    config: Res<Config>,
) {
    ga.initialize_population(config.population_size);
    ga.reproduction();
    start_evaluating_events.send(StartEvaluatingEvent)
}

fn genetic_algorithm_system(
    mut ga: ResMut<CreatureSpeciesGA>,
    mut start_evaluating_events: EventWriter<StartEvaluatingEvent>,
    mut generation_count: ResMut<GenerationCount>,
    config: Res<Config>,
) {
    if ga.all_have_finished_evaluating(config.population_size) {
        ga.save_results(generation_count.0);

        ga.replacement();
        ga.selection(config.population_size);
        ga.reproduction();
        generation_count.0 += 1;
        start_evaluating_events.send(StartEvaluatingEvent);
    }
}

fn count_finished_evaluating(
    mut finished_evaluating_events: EventReader<FinishedEvaluatingEvent>,
    mut ga: ResMut<CreatureSpeciesGA>,
) {
    for event in finished_evaluating_events.iter() {
        ga.finished_evaluating(event.chromosome.clone());
    }
}
