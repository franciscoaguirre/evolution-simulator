use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{BufWriter, Write},
};

use ron::ser::{to_string_pretty, PrettyConfig};

use crate::genetic_algorithm::operations::{Individual, Selective};

use super::runner::Runnable;

const MAX_UNCHANGED_GENERATIONS: usize = 10;
const IMPROVEMENT_THRESHOLD: f32 = 0.05;

#[derive(Default)]
pub struct SpeciesBasedAlgorithm<T: Individual + Selective> {
    pub population: HashMap<usize, Vec<T>>,
    pub offspring_population: Vec<T>,
    previous_best_by_species: HashMap<usize, (f32, usize)>,
    new_population: Vec<T>,
}

impl<T: Individual + Selective> SpeciesBasedAlgorithm<T> {
    fn produce_children(first_parent: &T, second_parent: &T, offspring_population: &mut Vec<T>) {
        let (mut first_child, mut second_child) = first_parent.breed(&second_parent);

        first_child = first_child.mutate(1.0);
        second_child = second_child.mutate(1.0);
        first_child.correct();
        second_child.correct();
        offspring_population.push(first_child);
        offspring_population.push(second_child);
    }
}

impl<T: Individual + Selective + fmt::Debug> Runnable<T> for SpeciesBasedAlgorithm<T> {
    fn initialize_population(&mut self, population_size: usize) {
        let initial_population: Vec<T> = (0..population_size).map(|_| T::random()).collect();

        for chromosome in initial_population {
            self.population
                .entry(chromosome.characteristic())
                .or_insert(Vec::new())
                .push(chromosome);
        }
    }

    fn selection(&mut self, population_size: usize) {
        let population = self.population.clone();
        self.population.clear();
        let mut flatten_population = population.values().flatten().collect::<Vec<&T>>();

        flatten_population.sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());
        flatten_population.truncate(population_size);

        for individual in flatten_population {
            self.population
                .entry(individual.characteristic())
                .or_insert(Vec::new())
                .push(individual.clone());
        }
    }

    // TODO: We are cloning every child multiple times
    fn reproduction(&mut self) {
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
                    > MAX_UNCHANGED_GENERATIONS
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
                    );
                }
            } else {
                if self
                    .previous_best_by_species
                    .entry(*species_size)
                    .or_insert((0.0, 0))
                    .0
                    < species[0].get_fitness() + IMPROVEMENT_THRESHOLD
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
                .or_insert(Vec::new())
                .push(chromosome.clone());
        }
        self.new_population.clear();
    }

    fn finished_evaluating(&mut self, chromosome: T) {
        self.new_population.push(chromosome);
    }

    fn all_have_finished_evaluating(&self, population_size: usize) -> bool {
        self.new_population.len() == population_size * 2
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

        stream.write(b"Best: ").unwrap();
        stream
            .write(
                to_string_pretty(&best, pretty_config.clone())
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        stream.write(b"\n").unwrap();
        stream.write(b"Worst: ").unwrap();
        stream
            .write(
                to_string_pretty(&worst, pretty_config.clone())
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        stream.write(b"\n").unwrap();
        stream.write(b"Median: ").unwrap();
        stream
            .write(
                to_string_pretty(median, pretty_config.clone())
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        stream.write(b"\n").unwrap();
        stream.write(b"Mean: ").unwrap();
        stream.write(mean_string.as_bytes()).unwrap();
        stream.write(b"\n").unwrap();
        stream.write(b"Std. Dev.: ").unwrap();
        stream.write(std_dev_string.as_bytes()).unwrap();
        stream.write(b"\n").unwrap();
        stream.flush().unwrap();
    }
}
