use bevy::prelude::*;

use super::{creature_chromosome::CreatureChromosome, runner::Algorithm};

pub const POPULATION_SIZE: usize = 100;

pub struct GeneticAlgorithmPlugin;

pub struct StartEvaluatingEvent {
    pub chromosome: CreatureChromosome,
}

pub struct FinishedEvaluatingEvent {
    pub chromosome: CreatureChromosome,
}

#[derive(Default)]
struct FinishedEvaluatingCounter(usize);

type CreatureGA = Algorithm<CreatureChromosome>;

impl Plugin for GeneticAlgorithmPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Algorithm::<CreatureChromosome>::default())
            .insert_resource(FinishedEvaluatingCounter::default())
            .add_event::<StartEvaluatingEvent>()
            .add_event::<FinishedEvaluatingEvent>()
            .add_startup_system(setup_genetic_algorithm.system())
            .add_system(count_finished_evaluating.system())
            .add_system(genetic_algorithm_system.system());
    }
}

fn setup_genetic_algorithm(
    mut ga: ResMut<CreatureGA>,
    mut start_evaluating_events: EventWriter<StartEvaluatingEvent>,
) {
    ga.initialize_population(POPULATION_SIZE);
    ga.selection();
    ga.reproduction();
    for chromosome in ga.population.iter().chain(ga.offspring_population.iter()) {
        start_evaluating_events.send(StartEvaluatingEvent {
            chromosome: chromosome.clone(),
        })
    }
}

fn genetic_algorithm_system(
    mut ga: ResMut<CreatureGA>,
    mut start_evaluating_events: EventWriter<StartEvaluatingEvent>,
) {
    if ga.all_have_finished_evaluating() {
        ga.replacement();
        ga.selection();
        ga.reproduction();
        for chromosome in ga.population.iter().chain(ga.offspring_population.iter()) {
            start_evaluating_events.send(StartEvaluatingEvent {
                chromosome: chromosome.clone(),
            })
        }
    }
}

fn count_finished_evaluating(
    mut finished_evaluating_events: EventReader<FinishedEvaluatingEvent>,
    mut ga: ResMut<CreatureGA>,
) {
    for event in finished_evaluating_events.iter() {
        println!("{:?}", event.chromosome);
        ga.finished_evaluating(event.chromosome.clone());
    }
}
