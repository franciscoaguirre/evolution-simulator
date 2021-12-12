use bevy::{app::AppExit, prelude::*};
use structopt::StructOpt;

use crate::{
    arguments::Opt,
    simulation2d::{
        events::{FinishedEvaluatingEvent, StartEvaluatingEvent},
        resources::GenerationCount,
    },
};

use super::{
    algorithms::{
        algorithm::Algorithm, runner::Runnable, species_based_algorithm::SpeciesBasedAlgorithm,
    },
    creature_chromosome::CreatureChromosome,
};

pub struct GeneticAlgorithmPlugin;

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
                algorithm: Box::new(CreatureSpeciesGA::new(
                    options.population_size,
                    options.max_generations,
                    options.max_no_improvement,
                    options.mutation_chance,
                    options.crossover_chance,
                )),
            });
        } else {
            println!("Running regular speciesism GA");
            app.insert_resource(GeneticAlgorithm {
                algorithm: Box::new(CreatureGA::new(
                    options.population_size,
                    options.max_generations,
                    options.max_no_improvement,
                    options.mutation_chance,
                    options.crossover_chance,
                )),
            });
        }
        app.insert_resource(FinishedEvaluatingCounter::default())
            .add_startup_system(setup_genetic_algorithm.system())
            .add_system(count_finished_evaluating.system())
            .add_system(genetic_algorithm_system.system())
            .add_system(check_should_end_simulation.system());
    }
}

fn setup_genetic_algorithm(
    mut ga: ResMut<GeneticAlgorithm>,
    mut start_evaluating_events: EventWriter<StartEvaluatingEvent>,
) {
    ga.algorithm.initialize_population();
    ga.algorithm.reproduction();
    start_evaluating_events.send(StartEvaluatingEvent {
        chromosomes: ga.algorithm.get_population_for_sim(),
    })
}

fn genetic_algorithm_system(
    mut ga: ResMut<GeneticAlgorithm>,
    mut start_evaluating_events: EventWriter<StartEvaluatingEvent>,
    mut generation_count: ResMut<GenerationCount>,
) {
    if ga.algorithm.all_have_finished_evaluating() {
        ga.algorithm.save_results(generation_count.0);

        ga.algorithm.replacement();
        ga.algorithm.selection();
        ga.algorithm.reproduction();
        generation_count.0 += 1;
        start_evaluating_events.send(StartEvaluatingEvent {
            chromosomes: ga.algorithm.get_population_for_sim(),
        });
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

fn check_should_end_simulation(
    ga: Res<GeneticAlgorithm>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if ga.algorithm.get_should_end() {
        app_exit_events.send(AppExit);
    };
}
