use bevy::prelude::*;
use structopt::StructOpt;

use crate::{
    arguments::Opt,
    simulation2d::resources::{Config, GenerationCount},
};

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

pub struct GeneticAlgorithm {
    pub algorithm: Box<dyn Runnable<CreatureChromosome> + Send + Sync>,
}

impl Plugin for GeneticAlgorithmPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let options = Opt::from_args();

        if options.speciesism {
            println!("Running speciesism GA");
            app.insert_resource(GeneticAlgorithm {
                algorithm: Box::new(CreatureSpeciesGA::default()),
            });
        } else {
            println!("Running regular speciesism GA");
            app.insert_resource(GeneticAlgorithm {
                algorithm: Box::new(CreatureGA::default()),
            });
        }
        app.insert_resource(FinishedEvaluatingCounter::default())
            .add_event::<StartEvaluatingEvent>()
            .add_event::<FinishedEvaluatingEvent>()
            .add_startup_system(setup_genetic_algorithm.system())
            .add_system(count_finished_evaluating.system())
            .add_system(genetic_algorithm_system.system());
    }
}

fn setup_genetic_algorithm(
    mut ga: ResMut<GeneticAlgorithm>,
    mut start_evaluating_events: EventWriter<StartEvaluatingEvent>,
    config: Res<Config>,
) {
    ga.algorithm.initialize_population(config.population_size);
    ga.algorithm.reproduction();
    start_evaluating_events.send(StartEvaluatingEvent)
}

fn genetic_algorithm_system(
    mut ga: ResMut<GeneticAlgorithm>,
    mut start_evaluating_events: EventWriter<StartEvaluatingEvent>,
    mut generation_count: ResMut<GenerationCount>,
    config: Res<Config>,
) {
    if ga
        .algorithm
        .all_have_finished_evaluating(config.population_size)
    {
        ga.algorithm.save_results(generation_count.0);

        ga.algorithm.replacement();
        ga.algorithm.selection(config.population_size);
        ga.algorithm.reproduction();
        generation_count.0 += 1;
        start_evaluating_events.send(StartEvaluatingEvent);
    }
}

fn count_finished_evaluating(
    mut finished_evaluating_events: EventReader<FinishedEvaluatingEvent>,
    mut ga: ResMut<GeneticAlgorithm>,
) {
    for event in finished_evaluating_events.iter() {
        ga.algorithm.finished_evaluating(event.chromosome.clone());
    }
}
