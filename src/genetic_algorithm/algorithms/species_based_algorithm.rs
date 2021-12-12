use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{BufWriter, Write},
};

use ron::ser::{to_string_pretty, PrettyConfig};

use crate::{
    config::CONFIG,
    genetic_algorithm::operations::{Individual, Selective},
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
    ) -> Self {
        SpeciesBasedAlgorithm {
            population_size,
            max_generations,
            max_no_improvement,
            mutation_chance,
            crossover_chance,
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

    fn initialize_population(&mut self) {
        self.current_generation = 0;
        self.current_unbeat_best = (std::f32::MIN, 0);
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
        }

        self.new_population.push(chromosome);
    }

    fn all_have_finished_evaluating(&self) -> bool {
        self.new_population.len() == self.population_size * 2
    }

    fn save_results(&self, generation_count: usize) {
        let mut population: Vec<T> = self.new_population.clone();
        population.sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());

        let pretty_config = PrettyConfig::default();
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

        let buffer = File::create(format!("results_generation_{}.ron", generation_count)).unwrap();
        let mut stream = BufWriter::new(buffer);

        println!(
            "Generation {}: Best: {}, Worst: {}, Median: {}, Mean: {}, StdDev: {}",
            generation_count,
            best.get_fitness(),
            worst.get_fitness(),
            median.get_fitness(),
            mean_string,
            std_dev_string
        );

        stream.write_all(b"Best: ").unwrap();
        stream
            .write_all(
                to_string_pretty(&best, pretty_config.clone())
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        stream.write_all(b"\n").unwrap();
        stream.write_all(b"Worst: ").unwrap();
        stream
            .write_all(
                to_string_pretty(&worst, pretty_config.clone())
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        stream.write_all(b"\n").unwrap();
        stream.write_all(b"Median: ").unwrap();
        stream
            .write_all(to_string_pretty(median, pretty_config).unwrap().as_bytes())
            .unwrap();
        stream.write_all(b"\n").unwrap();
        stream.write_all(b"Mean: ").unwrap();
        stream.write_all(mean_string.as_bytes()).unwrap();
        stream.write_all(b"\n").unwrap();
        stream.write_all(b"Std. Dev.: ").unwrap();
        stream.write_all(std_dev_string.as_bytes()).unwrap();
        stream.write_all(b"\n").unwrap();
        stream.flush().unwrap();
    }

    fn get_should_end(&self) -> bool {
        self.max_generations + 1 == self.current_generation
            || self.current_generation - self.current_unbeat_best.1 > self.max_no_improvement
    }
}
