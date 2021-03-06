use std::{collections::HashMap, fmt};

use bevy::log::info;

use crate::{
    config::CONFIG,
    genetic_algorithm::{
        operations::{Individual, Selective},
        write_stat::{write_stat, InstanceStats},
    },
};

use super::runner::Runnable;

#[derive(Default)]
pub struct SpeciesBasedAlgorithm<T: Individual + Selective> {
    pub population: HashMap<usize, Vec<T>>,
    pub offspring_population: Vec<T>,
    max_generations: usize,
    max_no_improvement: usize,
    current_unbeat_best: (f32, usize),
    previous_best_by_species: HashMap<usize, (f32, usize)>,
    new_population: Vec<T>,
    current_generation: usize,
    mutation_chance: f32,
    crossover_chance: f32,
    population_size: usize,
    testing: bool,
    max_test_count: usize,
    instance_number: usize,
    testing_count: usize,
    instance_stats: InstanceStats,
}

impl<T: Individual + Selective> SpeciesBasedAlgorithm<T> {
    fn produce_children(
        first_parent: &T,
        second_parent: &T,
        offspring_population: &mut Vec<T>,
        mutation_chance: f32,
        crossover_chance: f32,
    ) {
        let (mut first_child, mut second_child) =
            first_parent.breed(second_parent, crossover_chance);

        first_child = first_child.mutate(mutation_chance);
        second_child = second_child.mutate(mutation_chance);
        first_child.correct();
        second_child.correct();
        offspring_population.push(first_child);
        offspring_population.push(second_child);
    }
}

impl<T: Individual + Selective + Default> SpeciesBasedAlgorithm<T> {
    pub fn new(
        population_size: usize,
        max_generations: usize,
        max_no_improvement: usize,
        mutation_chance: f32,
        crossover_chance: f32,
        testing: bool,
        max_test_count: usize,
        instance_number: usize,
    ) -> Self {
        SpeciesBasedAlgorithm {
            population_size,
            max_generations,
            max_no_improvement,
            mutation_chance,
            crossover_chance,
            testing,
            max_test_count,
            instance_number,
            ..Default::default()
        }
    }
}

impl<T: Individual + Selective + fmt::Debug> Runnable<T> for SpeciesBasedAlgorithm<T> {
    fn get_population_for_sim(&self) -> Vec<T> {
        self.population
            .values()
            .flatten()
            .chain(self.offspring_population.iter())
            .cloned()
            .collect()
    }

    fn is_testing(&self) -> bool {
        self.testing
    }

    fn should_finish_testing(&self) -> bool {
        self.testing_count >= self.max_test_count
    }

    fn initialize_population(&mut self) {
        self.instance_stats = InstanceStats::default();
        self.current_generation = 0;
        self.testing_count += 1;
        self.current_unbeat_best = (std::f32::MIN, 0);
        self.population.clear();
        self.offspring_population.clear();
        self.new_population.clear();
        self.previous_best_by_species.clear();

        let initial_population: Vec<T> = (0..self.population_size).map(|_| T::random()).collect();

        for chromosome in initial_population {
            self.population
                .entry(chromosome.characteristic())
                .or_insert_with(Vec::new)
                .push(chromosome);
        }
    }

    fn selection(&mut self) {
        let population = self.population.clone();
        self.population.clear();
        let mut flatten_population = population.values().flatten().collect::<Vec<&T>>();

        flatten_population.sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());
        flatten_population.truncate(self.population_size);

        for individual in flatten_population {
            self.population
                .entry(individual.characteristic())
                .or_insert_with(Vec::new)
                .push(individual.clone());
        }
    }

    // TODO: We are cloning every child multiple times
    fn reproduction(&mut self) {
        self.current_generation += 1;

        let mut offspring_population: Vec<T> = Vec::new();

        // Create reference to elements ordered by fitness
        // Will be used for breeding when a species has not improved for a while
        let population = self.population.clone();

        let mut population: Vec<&T> = population.values().flatten().collect();
        population.sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());

        for (species_size, species) in self.population.iter_mut() {
            // If the species has not improved for a while, breed it with the best individual
            // In the first run there is no best_by_species
            if !self.previous_best_by_species.is_empty()
                && self
                    .previous_best_by_species
                    .get(species_size)
                    .unwrap_or(&(0.0, 0))
                    .1
                    > CONFIG.max_unchanged_generations
            {
                for (first_parent, second_parent) in species
                    .iter()
                    .zip(population.iter())
                    .take(species.len() / 2)
                {
                    SpeciesBasedAlgorithm::<T>::produce_children(
                        first_parent,
                        second_parent,
                        &mut offspring_population,
                        self.mutation_chance,
                        self.crossover_chance,
                    );
                }
            } else {
                if self
                    .previous_best_by_species
                    .entry(*species_size)
                    .or_insert((0.0, 0))
                    .0
                    < species[0].get_fitness() + CONFIG.improvement_threshold
                {
                    self.previous_best_by_species
                        .insert(*species_size, (species[0].get_fitness(), 0));
                } else {
                    self.previous_best_by_species
                        .entry(*species_size)
                        .and_modify(|entry| {
                            entry.1 += 1;
                        });
                }

                // Breed the species
                for parents in species.chunks(2) {
                    if parents.len() < 2 {
                        continue;
                    }

                    let first_parent = &parents[0];
                    let second_parent = &parents[1];

                    SpeciesBasedAlgorithm::<T>::produce_children(
                        first_parent,
                        second_parent,
                        &mut offspring_population,
                        self.mutation_chance,
                        self.crossover_chance,
                    );
                }
            }

            if species.len() % 2 != 0 {
                let first_parent = &species.last().unwrap();
                let second_parent = population[species.len() - 1];

                SpeciesBasedAlgorithm::<T>::produce_children(
                    first_parent,
                    second_parent,
                    &mut offspring_population,
                    self.mutation_chance,
                    self.crossover_chance,
                );
            }
        }

        offspring_population.truncate(self.population.values().flatten().count());
        self.offspring_population = offspring_population;
    }

    fn replacement(&mut self) {
        self.population.clear();
        for chromosome in self.new_population.iter() {
            self.population
                .entry(chromosome.characteristic())
                .or_insert_with(Vec::new)
                .push(chromosome.clone());
        }
        self.new_population.clear();
    }

    fn finished_evaluating(&mut self, chromosome: T) {
        if chromosome.get_fitness() > self.current_unbeat_best.0 {
            self.current_unbeat_best = (chromosome.get_fitness(), self.current_generation);

            if self.testing {
                self.instance_stats.best_fitness = chromosome.get_fitness();
                self.instance_stats.best_fitness_sum += chromosome.get_fitness();
                self.instance_stats.generation_count = self.current_generation;
            }
        }

        self.new_population.push(chromosome);
    }

    fn all_have_finished_evaluating(&self) -> bool {
        self.new_population.len() == self.population_size * 2
    }

    fn save_results(&self, generation_count: usize) {
        let mut population: Vec<T> = self.new_population.clone();
        population.sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());

        let best = population[0].clone();
        let worst = population.last().unwrap();
        let median = &population[population.len() / 2];

        let mean =
            population.iter().map(|x| x.get_fitness()).sum::<f32>() / population.len() as f32;
        let std_dev = (population
            .iter()
            .map(|x| (x.get_fitness() - mean).powi(2))
            .sum::<f32>()
            / population.len() as f32)
            .sqrt();

        let mean_string = format!("{:.2}", mean);
        let std_dev_string = format!("{:.2}", std_dev);

        println!(
            "Generation {}: Best: {}, Worst: {}, Median: {}, Mean: {}, StdDev: {}",
            generation_count,
            best.get_fitness(),
            worst.get_fitness(),
            median.get_fitness(),
            mean_string,
            std_dev_string
        );

        if self.testing {
            write_stat(
                format!(
                    "experiments/population_{}_mutation_{}_crossover_{}/instance_{}/execution_{}/generation_{}.ron",
                    self.new_population.len(),
                    self.mutation_chance,
                    self.crossover_chance,
                    self.instance_number,
                    self.testing_count,
                    generation_count
                ),
                best.get_fitness(),
                median.get_fitness(),
                worst.get_fitness(),
                mean,
                std_dev,
            );

            self.instance_stats.write(format!(
                "experiments/population_{}_mutation_{}_crossover_{}/instance_{}/execution_{}/stats.ron",
                self.new_population.len(),
                self.mutation_chance,
                self.crossover_chance,
                self.instance_number,
                self.testing_count,
            ));
        }
    }

    fn get_should_end(&self) -> bool {
        self.max_generations + 1 == self.current_generation
            || self.current_generation - self.current_unbeat_best.1 > self.max_no_improvement
    }
}
